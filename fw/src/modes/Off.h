#ifndef CONTROLLER_OFF_H
#define CONTROLLER_OFF_H

#include "../api.h"

class Off : public Mode {
public:
    static const constexpr Type TYPE = Type::OFF;

    struct Config {
    } __attribute__((packed));

    Off() {}
    Off(const Config& config) {}

    virtual void render(CRGB(& leds)[LED_NUM]) override {
        // Fade out smoothly
        fadeToBlackBy(leds, LED_NUM, 32);
    }
};


#endif //CONTROLLER_OFF_H
