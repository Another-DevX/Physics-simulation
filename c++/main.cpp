#include "include/engine.h"
#include "include/lorenz_attractor.h"
#include <memory>
#include <iostream>

int main() {
    const uint32_t WINDOW_WIDTH = 1080;
    const uint32_t WINDOW_HEIGHT = 720;
    const std::string WINDOW_TITLE = "Particle Simulation in C++";

    try {
        LorenzAttractor scene;
        Engine engine(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT);
        engine.run(scene);
    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << '\n';
        return EXIT_FAILURE;
    }

    return EXIT_SUCCESS;
}
