#include "toy.h"

uint32_t Toy::add_one(const uint32_t x) {
    return x + 1;
}

Toy::Toy(bool enabled) : enabled(enabled) {

}

Toy::Toy() : enabled(true) {

}

Toy::~Toy() {

}