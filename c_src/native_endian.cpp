#include <bit>

extern "C" {
  //**
  //* Returns CPU native endian type.
  //*
  //* - Little endian: returns 1.
  //* - Big endian: returns 2.
  //* - Neither little nor big: 3. (PDP endian, middle endian, )
  //*
  int native_() {
    if constexpr (std::endian::native == std::endian::little) {
      return 1;
    } else if constexpr (std::endian::native == std::endian::big) {
      return 2;
    } else {
      return 3;
    }
  }
}
