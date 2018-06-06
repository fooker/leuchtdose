#ifndef CONTROLLER_EMERGENCY_H
#define CONTROLLER_EMERGENCY_H

#include "../api.h"

class Emergency : public Mode {
public:
    static const constexpr Type TYPE = Type::EMERGENCY;

    enum class Direction: uint8_t {
        LEFT = 0,
        RIGHT = 1,
    };

    struct Config {
        uint8_t r, g, b;
        Direction direction;
        uint8_t speed;
    } __attribute__((packed));

    Emergency(const Config& config) :
            color(config.r, config.g, config.b),
            direction(config.direction),
            speed(config.speed) {
    }

    virtual void render(CRGB(& leds)[LED_NUM]) override {
        // Fade out to get some trail
        fadeToBlackBy(leds, LED_NUM, 64);

        leds[this->pos / speed * 4 + 0] = this->color;
        leds[this->pos / speed * 4 + 1] = this->color;
        leds[this->pos / speed * 4 + 2] = this->color;
        leds[this->pos / speed * 4 + 3] = this->color;

        switch (this->direction) {
            case Direction::LEFT:
                this->pos = (this->pos + 1) % (6 * this->speed);
                break;

            case Direction::RIGHT:
                this->pos = (this->pos - 1) % (6 * this->speed);
                break;
        }
    }

private:
    const CRGB color;
    const Direction direction;
    const uint8_t speed;

    uint8_t pos;
};


#endif //CONTROLLER_EMERGENCY_H
