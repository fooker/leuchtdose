#include <Arduino.h>
#include <FastLED.h>


#include "config.h"
#include "api.h"
#include "modes/Off.h"
#include "modes/Emergency.h"
#include "modes/Sparkle.h"
#include "modes/Solid.h"
#include "modes/Breath.h"
#include "modes/Rings.h"
#include "api.h"


CRGB leds[LED_NUM];



static char data[64];

Type type;
Mode* mode = nullptr;


template<class M>
Mode* load(Stream& stream) {
    static_assert(sizeof(M) < sizeof(data), "data not large enough");

    typename M::Config config;
    const size_t i = stream.readBytes((char*) &config, sizeof(typename M::Config));
    if (i != sizeof(typename M::Config)) {
        return new ((void*) data) Off();
    }

    return new ((void*) data) M(config);
}

Mode* load(Stream& stream) {
    switch (type = (Type) Serial.read()) {
        case Type::OFF:
            return load<Off>(Serial);

        case Type::SOLID:
            return load<Solid>(Serial);

        case Type::BREATH:
            return load<Breath>(Serial);

        case Type::RINGS:
            return load<Rings>(Serial);

        case Type::EMERGENCY:
            return load<Emergency>(Serial);

        case Type::SPARKLE:
            return load<Sparkle>(Serial);
    }
}


void setup() {
    delay(3000);

    FastLED.addLeds<LED_TYPE, LED_PIN, LED_ORDER>(leds, LED_NUM)
            .setCorrection(TypicalLEDStrip)
            .setDither(BINARY_DITHER);
    FastLED.setBrightness(128);

    Serial.begin(115200);
}

void loop() {
    if (Serial.available()) {
        mode = load(Serial);
    }

    EVERY_N_MILLISECONDS(10) {
        if (mode != nullptr) {
            mode->render(leds);
        }
    }
    FastLED.show();
}
