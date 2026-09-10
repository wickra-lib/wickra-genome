// Wickra Genome — C++ wrapper over the C ABI.
//
// Header-only, C++17, no dependency beyond the standard library and
// `wickra_genome.h` beside it. Link the same `wickra_genome` library the C
// binding does.
//
// What it adds over calling the C functions directly is the handling nobody
// wants to write twice: the handle is owned and freed, the two-call length
// protocol behind `wickra_genome_command` is done for you, and a failure comes
// back as an exception rather than a negative integer a caller can ignore.
//
//     #include <wickra_genome.hpp>
//
//     wickra::Genome handle(R"({"universe":["AAA"], ... })");
//     std::string report = handle.command(R"({"cmd":"scan","data":{...}})");
//
// The genome is data-driven, so this wrapper deliberately stops at strings:
// the spec and the report are JSON, and which JSON library a caller uses is
// their choice, not this header's.

#ifndef WICKRA_SCREENER_HPP
#define WICKRA_SCREENER_HPP

#include <cstddef>
#include <cstdint>
#include <stdexcept>
#include <string>
#include <utility>

#include "wickra_genome.h"

namespace wickra {

/// Thrown when the library rejects a spec or a command.
class GenomeError : public std::runtime_error {
 public:
  explicit GenomeError(const std::string& what) : std::runtime_error(what) {}
};

/// An owning handle to a genome built from a scan spec.
///
/// Move-only, because the underlying handle is a unique resource: copying it
/// would free the same pointer twice.
class Genome {
 public:
  /// Build a genome from a spec JSON string.
  ///
  /// Throws `GenomeError` if the spec is not valid JSON or not a valid spec.
  explicit Genome(const std::string& spec_json)
      : handle_(wickra_genome_new(spec_json.c_str())) {
    if (handle_ == nullptr) {
      throw GenomeError("wickra_genome_new rejected the spec");
    }
  }

  ~Genome() { wickra_genome_free(handle_); }

  Genome(const Genome&) = delete;
  Genome& operator=(const Genome&) = delete;

  Genome(Genome&& other) noexcept : handle_(other.handle_) {
    other.handle_ = nullptr;
  }

  Genome& operator=(Genome&& other) noexcept {
    if (this != &other) {
      wickra_genome_free(handle_);
      handle_ = other.handle_;
      other.handle_ = nullptr;
    }
    return *this;
  }

  /// Apply a command JSON and return the response JSON.
  ///
  /// The C entry point writes into a caller buffer and reports the length it
  /// needed, so this asks for the length first and then reads. A command the
  /// library understands but cannot carry out answers in band with
  /// `{"ok":false,"error":...}`; a negative return is a failure of the call
  /// itself and becomes an exception.
  std::string command(const std::string& cmd_json) {
    const std::int32_t needed =
        wickra_genome_command(handle_, cmd_json.c_str(), nullptr, 0);
    if (needed < 0) {
      throw GenomeError("wickra_genome_command failed with code " +
                          std::to_string(needed));
    }

    std::string out(static_cast<std::size_t>(needed), '\0');
    // The C side writes a trailing NUL, so the buffer has to hold one more byte
    // than the response itself.
    const std::int32_t written = wickra_genome_command(
        handle_, cmd_json.c_str(), out.data(),
        static_cast<std::uintptr_t>(out.size()) + 1);
    if (written < 0) {
      throw GenomeError("wickra_genome_command failed with code " +
                          std::to_string(written));
    }
    if (written != needed) {
      // The response changed length between the two calls, which cannot happen
      // for a handle only this thread is using. Saying so is better than
      // returning a string that is half of one answer and half of another.
      throw GenomeError("wickra_genome_command length changed between calls");
    }
    return out;
  }

  /// The library version.
  static std::string version() { return std::string(wickra_genome_version()); }

 private:
  WickraGenome* handle_;
};

}  // namespace wickra

#endif  // WICKRA_SCREENER_HPP
