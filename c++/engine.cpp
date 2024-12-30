#include "../include/engine.h"
#include <stdexcept>
#include <thread>

Engine::Engine(const std::string& title, uint32_t width, uint32_t height)
    : globalContext(width, height) {
    if (SDL_Init(SDL_INIT_VIDEO) < 0) {
        throw std::runtime_error("Failed to initialize SDL: " + std::string(SDL_GetError()));
    }

    window = SDL_CreateWindow(title.c_str(), SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED,
                              width, height, SDL_WINDOW_SHOWN);
    if (!window) {
        throw std::runtime_error("Failed to create SDL window: " + std::string(SDL_GetError()));
    }

    renderer = SDL_CreateRenderer(window, -1, SDL_RENDERER_ACCELERATED);
    if (!renderer) {
        throw std::runtime_error("Failed to create SDL renderer: " + std::string(SDL_GetError()));
    }

    previousInstant = std::chrono::high_resolution_clock::now();
}

Engine::~Engine() {
    if (renderer) SDL_DestroyRenderer(renderer);
    if (window) SDL_DestroyWindow(window);
    SDL_Quit();
}

void Engine::run(Scene& scene) {
    bool running = true;

    while (running) {
        SDL_Event event;
        while (SDL_PollEvent(&event)) {
            if (event.type == SDL_QUIT) {
                running = false;
            }
            scene.handleEvent(globalContext, event);
        }

        if (scene.isDone()) {
            break;
        }

        auto now = std::chrono::high_resolution_clock::now();
        std::chrono::duration<float> deltaTime = now - previousInstant;
        previousInstant = now;

        scene.update(globalContext, deltaTime.count());

        SDL_SetRenderDrawColor(renderer, 0, 0, 0, 255);
        SDL_RenderClear(renderer);

        scene.render(globalContext, renderer);

        SDL_RenderPresent(renderer);

        std::this_thread::sleep_for(std::chrono::milliseconds(16));
    }
}
