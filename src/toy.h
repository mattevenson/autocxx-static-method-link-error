    #pragma once

    #include <cstdint>

    class Toy {
        private:
            bool enabled;
        public:
            Toy();
            Toy(bool enabled);
            ~Toy();
            static uint32_t add_one(const uint32_t x);
        };