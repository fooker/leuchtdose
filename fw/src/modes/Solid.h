#ifndef CONTROLLER_SOLID_H
#define CONTROLLER_SOLID_H

#include "../api.h"

class Solid : public Mode {
public:
    static const constexpr Type TYPE = Type::SOLID;

    struct Config {
        uint8_t r, g, b;
    } __attribute__((packed));

    Solid(const Config& config) :
            color(config.r, config.g, config.b) {
    }

    virtual void render(CRGB(& leds)[LED_NUM]) override {
        fill_solid(leds, LED_NUM, color);
    }

private:
    const CRGB color;
};


#endif //CONTROLLER_SOLID_H
