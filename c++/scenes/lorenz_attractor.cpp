#include "../include/lorenz_attractor.h"
#include "../include/utils/rk4.h"
#include <SDL2/SDL.h>
#include <cmath>

LorenzAttractor::LorenzAttractor()
    : render_mode(RenderMode::Points), // Modo predeterminado: puntos blancos
      sigma(10.0f), beta(2.667f), rho(28.0f),
      solutions(std::nullopt), current_index(0), done(false),
      camera_rotation{0.0f, 0.0f}, is_mouse_down(false), zoom(1.0f)
{
    solve();
}

void LorenzAttractor::solve()
{
    auto lorenz = [this](float t, const std::array<float, 3> &state)
    {
        return this->lorenz(t, state);
    };

    auto r0 = std::array<float, 3>{0.0f, 1.0f, 1.05f};
    auto [t_values, w] = rk4(0.0f, 50.0f, r0, lorenz, 10000);
    solutions = {t_values, w};
    for (auto &sol : w)
    {
        printf("x: %f, y: %f, z: %f", sol[0], sol[1], sol[2]);
    }
}
void LorenzAttractor::setRenderMode(RenderMode mode)
{
    render_mode = mode;
}

std::array<float, 3> LorenzAttractor::lorenz(float t, const std::array<float, 3> &state)
{
    float x = state[0], y = state[1], z = state[2];
    return {sigma * (y - x), x * (rho - z) - y, x * y - beta * z};
}

std::pair<int, int> LorenzAttractor::project(const std::array<float, 3> &point, int width, int height)
{
    float d = 50.0f;
    float x_screen = point[0] / (point[2] + d) * 200.0f * zoom + width / 2.0f;
    float y_screen = point[1] / (point[2] + d) * 200.0f * zoom + height / 2.0f;
    return {static_cast<int>(x_screen), static_cast<int>(y_screen)};
}

std::array<float, 3> LorenzAttractor::rotate3D(const std::array<float, 3> &point)
{
    float x = point[0], y = point[1], z = point[2];
    float theta_x = camera_rotation.first;
    float theta_y = camera_rotation.second;

    float y1 = y * cos(theta_x) - z * sin(theta_x);
    float z1 = y * sin(theta_x) + z * cos(theta_x);

    float x2 = x * cos(theta_y) + z1 * sin(theta_y);
    float z2 = -x * sin(theta_y) + z1 * cos(theta_y);

    return {x2, y1, z2};
}

void LorenzAttractor::handleEvent(GlobalContext &ctx, const SDL_Event &event)
{
    switch (event.type)
    {
    case SDL_MOUSEBUTTONDOWN:
        if (event.button.button == SDL_BUTTON_LEFT)
        {
            is_mouse_down = true;
        }
        break;

    case SDL_MOUSEBUTTONUP:
        if (event.button.button == SDL_BUTTON_LEFT)
        {
            is_mouse_down = false;
        }
        break;

    case SDL_MOUSEMOTION:
        if (is_mouse_down)
        {
            camera_rotation.first += event.motion.yrel * 0.01f;  // Rotación en X
            camera_rotation.second += event.motion.xrel * 0.01f; // Rotación en Y
        }
        break;
    case SDL_KEYDOWN:
        switch (event.key.keysym.sym)
        {
        case SDLK_ESCAPE: // Escape
            done = true;
            break;
        case SDLK_r: // Reset
            current_index = 0;
            break;
        case SDLK_LEFT: // Disminuir velocidad
            ctx.simulation_speed = std::max(0.1f, ctx.simulation_speed - 0.1f);
            break;
        case SDLK_RIGHT: // Aumentar velocidad
            ctx.simulation_speed += 0.1f;
            break;
        case SDLK_m: // Cambiar modo de renderizado
            if (render_mode == RenderMode::Points)
            {
                render_mode = RenderMode::GradientLines;
            }
            else
            {
                render_mode = RenderMode::Points;
            }
            break;
        default:
            break;
        }
        break;

    case SDL_MOUSEWHEEL:
        if (event.wheel.y > 0)
        { // Zoom in
            zoom *= 1.1f;
        }
        else if (event.wheel.y < 0)
        { // Zoom out
            zoom *= 0.9f;
        }
        break;

    default:
        break;
    }
}
void LorenzAttractor::update(GlobalContext &ctx, float dt)
{
    if (solutions)
    {
        const auto &sol = solutions->second;
        current_index = std::min(current_index + static_cast<int>(ctx.simulation_speed * 10.0f), static_cast<int>(sol.size()) - 1);
    }
}

void LorenzAttractor::render(GlobalContext &ctx, SDL_Renderer *renderer)
{
    if (!solutions)
        return;

    const auto &sol = solutions->second;
    size_t total_points = sol.size();

    std::pair<int, int> last_projected;

    for (size_t i = 1; i < current_index; ++i)
    {
        auto point = sol[i];
        auto rotated = rotate3D(point);
        auto projected = project(rotated, ctx.screen_width, ctx.screen_height);

        if (render_mode == RenderMode::Points)
        {
            SDL_SetRenderDrawColor(renderer, 255, 255, 255, 255); // Blanco
            SDL_RenderDrawPoint(renderer, projected.first, projected.second);
        }
        else if (render_mode == RenderMode::GradientLines)
        {
            float t = static_cast<float>(i) / total_points;
            Uint8 r = static_cast<Uint8>(255.0f * (1.0f - t));
            Uint8 g = static_cast<Uint8>(255.0f * t);

            if (i > 1)
            {
                SDL_SetRenderDrawColor(renderer, r, g, 0, 255);
                SDL_RenderDrawLine(renderer, last_projected.first, last_projected.second,
                                   projected.first, projected.second);
            }
        }

        last_projected = projected;
    }
}

bool LorenzAttractor::isDone() const
{
    return done;
}
