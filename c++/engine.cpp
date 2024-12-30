#include "../include/engine.h"
#include <stdexcept>
#include <thread>

#ifdef __EMSCRIPTEN__
#include <emscripten.h>
#endif

Engine::Engine(const std::string &title, uint32_t width, uint32_t height)
    : globalContext(width, height)
{
    if (SDL_Init(SDL_INIT_VIDEO) < 0)
    {
        throw std::runtime_error("Failed to initialize SDL: " + std::string(SDL_GetError()));
    }

    window = SDL_CreateWindow(title.c_str(), SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED,
                              width, height, SDL_WINDOW_SHOWN);
    if (!window)
    {
        throw std::runtime_error("Failed to create SDL window: " + std::string(SDL_GetError()));
    }

    renderer = SDL_CreateRenderer(window, -1, SDL_RENDERER_ACCELERATED);
    if (!renderer)
    {
        throw std::runtime_error("Failed to create SDL renderer: " + std::string(SDL_GetError()));
    }

    previousInstant = std::chrono::high_resolution_clock::now();
}

Engine::~Engine()
{
    if (renderer)
        SDL_DestroyRenderer(renderer);
    if (window)
        SDL_DestroyWindow(window);
    SDL_Quit();
}

void Engine::run(Scene &scene)
{
    currentScene = &scene; // Configura la escena actual

#ifdef __EMSCRIPTEN__
    emscripten_set_main_loop_arg(mainloop, this, 0, 1);
#else
    while (running)
    {
        mainloop(this); // Llama a mainloop con el contexto del engine
    }
#endif
}

void Engine::mainloop(void *arg)
{
    auto *engine = static_cast<Engine *>(arg);

    SDL_Event event;
    while (SDL_PollEvent(&event))
    {
        if (event.type == SDL_QUIT)
        {
            engine->running = false;
#ifdef __EMSCRIPTEN__
            emscripten_cancel_main_loop(); // Detener el loop en WASM
#endif
            return;
        }
        engine->currentScene->handleEvent(engine->globalContext, event);
    }

    auto now = std::chrono::high_resolution_clock::now();
    auto deltaTime = std::chrono::duration<float>(now - engine->previousInstant).count();
    engine->previousInstant = now;

    engine->currentScene->update(engine->globalContext, deltaTime);

    SDL_SetRenderDrawColor(engine->renderer, 0, 0, 0, 255);
    SDL_RenderClear(engine->renderer);

    engine->currentScene->render(engine->globalContext, engine->renderer);

    SDL_RenderPresent(engine->renderer);

    SDL_Delay(16);
}