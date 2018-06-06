#ifndef CONTROLLER_API_H
#define CONTROLLER_API_H

#include <Arduino.h>
#include <FastLED.h>

#include "config.h"

enum class Type {
    OFF = 0,
    SOLID = 1,
    BREATH = 2,
    RINGS = 3,
    EMERGENCY = 4,
    SPARKLE = 5,
};


class Mode {
public:
    Mode() {}
    virtual ~Mode() {}

    static void* operator new(size_t size, void* data) {
        return data;
    }

    virtual void render(CRGB(& leds)[LED_NUM]) = 0;
};

#endif //CONTROLLER_API_H
