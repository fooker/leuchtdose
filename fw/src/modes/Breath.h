#ifndef CONTROLLER_BREATH_H
#define CONTROLLER_BREATH_H

#include "../api.h"

class Breath : public Mode {
public:
    static const constexpr Type TYPE = Type::BREATH;

    struct Config {
        uint8_t r, g, b;
        uint8_t speed;
    } __attribute__((packed));

    Breath(const Config& config) :
            color(config.r, config.g, config.b),
            speed(config.speed) {
    }

    virtual void render(CRGB(& leds)[LED_NUM]) override {
        // Stolen from the interwebz
        const uint8_t value = (exp(sin(millis() / (10000.0 / this->speed) * PI)) - 0.36787944) * 108.0;

        fill_solid(leds, LED_NUM, this->color % value);
    }

private:
    const CRGB color;
    const uint8_t speed;
};


#endif //CONTROLLER_BREATH_H
