#ifndef ENGINE_H
#define ENGINE_H

#include <SDL2/SDL.h>
#include <string>
#include <chrono>

struct GlobalContext
{
    float simulation_speed = 1.0f;
    bool paused = false;
    uint32_t screen_width;
    uint32_t screen_height;

    GlobalContext(uint32_t width, uint32_t height)
        : screen_width(width), screen_height(height) {}
};

class Scene
{
public:
    virtual void handleEvent(GlobalContext &ctx, const SDL_Event &event) = 0;
    virtual void update(GlobalContext &ctx, float dt) = 0;
    virtual void render(GlobalContext &ctx, SDL_Renderer *renderer) = 0;
    virtual bool isDone() const = 0;
    virtual ~Scene() = default;
};

class Engine
{
public:
    Engine(const std::string &title, uint32_t width, uint32_t height);
    ~Engine();
    void run(Scene &scene);

private:
    static void mainloop(void *arg);
    bool running = true;
    SDL_Window *window = nullptr;
    SDL_Renderer *renderer = nullptr;
    GlobalContext globalContext;
    std::chrono::time_point<std::chrono::high_resolution_clock> previousInstant;
    Scene *currentScene = nullptr; // Escena actual para el loop
};

#endif