#ifndef LORENZ_ATTRACTOR_H
#define LORENZ_ATTRACTOR_H

#include "engine.h"
#include <array>
#include <vector>
#include <optional>



enum class RenderMode {
    Points,
    GradientLines
};



class LorenzAttractor : public Scene {
public:
    LorenzAttractor();

    void handleEvent(GlobalContext& ctx, const SDL_Event& event) override;
    void update(GlobalContext& ctx, float dt) override;
    void render(GlobalContext& ctx, SDL_Renderer* renderer) override;
    bool isDone() const override;

    void setRenderMode(RenderMode mode);

private:
    RenderMode render_mode;
    float sigma, beta, rho;
    std::optional<std::pair<std::vector<float>, std::vector<std::array<float, 3>>>> solutions;
    int current_index;
    bool done;
    std::pair<float, float> camera_rotation;
    bool is_mouse_down;
    float zoom;

    void solve();
    std::array<float, 3> lorenz(float t, const std::array<float, 3>& state);
    std::pair<int, int> project(const std::array<float, 3>& point, int width, int height);
    std::array<float, 3> rotate3D(const std::array<float, 3>& point);
};

#endif
