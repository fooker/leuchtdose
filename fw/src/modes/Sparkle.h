#ifndef CONTROLLER_SPARKLE_H
#define CONTROLLER_SPARKLE_H

#include "../api.h"

class Sparkle : public Mode {
public:
    static const constexpr Type TYPE = Type::SPARKLE;

    struct Config {
        uint8_t fade;
        uint8_t chance;
        struct {
            uint8_t r, g, b;
        } palette[16];
    } __attribute__((packed));

    Sparkle(const Config& config) :
            fade(config.fade),
            chance(config.chance),
            palette(CRGB(config.palette[0x0].r, config.palette[0x0].g, config.palette[0x0].b),
                    CRGB(config.palette[0x1].r, config.palette[0x1].g, config.palette[0x1].b),
                    CRGB(config.palette[0x2].r, config.palette[0x2].g, config.palette[0x2].b),
                    CRGB(config.palette[0x3].r, config.palette[0x3].g, config.palette[0x3].b),
                    CRGB(config.palette[0x4].r, config.palette[0x4].g, config.palette[0x4].b),
                    CRGB(config.palette[0x5].r, config.palette[0x5].g, config.palette[0x5].b),
                    CRGB(config.palette[0x6].r, config.palette[0x6].g, config.palette[0x6].b),
                    CRGB(config.palette[0x7].r, config.palette[0x7].g, config.palette[0x7].b),
                    CRGB(config.palette[0x8].r, config.palette[0x8].g, config.palette[0x8].b),
                    CRGB(config.palette[0x9].r, config.palette[0x9].g, config.palette[0x9].b),
                    CRGB(config.palette[0xA].r, config.palette[0xA].g, config.palette[0xA].b),
                    CRGB(config.palette[0xB].r, config.palette[0xB].g, config.palette[0xB].b),
                    CRGB(config.palette[0xC].r, config.palette[0xC].g, config.palette[0xC].b),
                    CRGB(config.palette[0xD].r, config.palette[0xD].g, config.palette[0xD].b),
                    CRGB(config.palette[0xE].r, config.palette[0xE].g, config.palette[0xE].b),
                    CRGB(config.palette[0xF].r, config.palette[0xF].g, config.palette[0xF].b)) {
    }

    virtual void render(CRGB(& leds)[LED_NUM]) override {
        // Fade out smoothly
        fadeToBlackBy(leds, LED_NUM, this->fade);

        // Blend in new pixel
        if (random8(0xFF) < this->chance) {
            const int i = random16(LED_NUM);
            leds[i] = ColorFromPalette(this->palette, random8());
        }
    }

private:
    const uint8_t fade;
    const uint8_t chance;
    const CRGBPalette16 palette;
};


#endif //CONTROLLER_SPARKLE_H
