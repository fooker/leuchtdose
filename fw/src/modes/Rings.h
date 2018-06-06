#ifndef CONTROLLER_RINGS_H
#define CONTROLLER_RINGS_H

#include "../api.h"

class Rings : public Mode {
public:
    static const constexpr Type TYPE = Type::RINGS;

    enum class Direction: uint8_t {
        DOWN = 0,
        UP = 1,
    };

    struct Config {
        uint8_t r, g, b;
        Direction direction;
        uint8_t speed;
    } __attribute__((packed));

    Rings(const Config& config) :
            color(config.r, config.g, config.b),
            direction(config.direction),
            speed(config.speed) {
    }

    virtual void render(CRGB(& leds)[LED_NUM]) override {
        // Fade out to get some trail
        fadeToBlackBy(leds, LED_NUM, 32);

        leds[this->pos / this->speed + 0 * 4] = this->color;
        leds[this->pos / this->speed + 1 * 4] = this->color;
        leds[this->pos / this->speed + 2 * 4] = this->color;
        leds[this->pos / this->speed + 3 * 4] = this->color;
        leds[this->pos / this->speed + 4 * 4] = this->color;
        leds[this->pos / this->speed + 5 * 4] = this->color;

        switch (this->direction) {
            case Direction::UP:
                this->pos = (this->pos + 1) % (4 * this->speed);
                break;

            case Direction::DOWN:
                this->pos = (this->pos - 1) % (4 * this->speed);
                break;
        }
    }

private:
    const CRGB color;
    const Direction direction;
    const uint8_t speed;

    uint8_t pos;
};


#endif //CONTROLLER_BREATH_H
