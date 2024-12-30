#include "../include/utils/rk4.h"

std::pair<std::vector<float>, std::vector<std::array<float, 3>>> rk4(
    float a, float b, const std::array<float, 3>& alpha,
    std::function<std::array<float, 3>(float, const std::array<float, 3>&)> f, 
    unsigned int n) {
    float h = (b - a) / n;
    std::vector<std::array<float, 3>> w = {alpha};
    std::vector<float> t_values = {a};

    for (unsigned int i = 1; i <= n; ++i) {
        float t = a + i * h;
        const auto& w_prev = w.back();

        auto k1 = f(t, w_prev);
        for (auto& val : k1) val *= h;

        auto k2 = f(t + h / 2.0f, {
            w_prev[0] + k1[0] / 2.0f,
            w_prev[1] + k1[1] / 2.0f,
            w_prev[2] + k1[2] / 2.0f
        });
        for (auto& val : k2) val *= h;

        auto k3 = f(t + h / 2.0f, {
            w_prev[0] + k2[0] / 2.0f,
            w_prev[1] + k2[1] / 2.0f,
            w_prev[2] + k2[2] / 2.0f
        });
        for (auto& val : k3) val *= h;

        auto k4 = f(t + h, {
            w_prev[0] + k3[0],
            w_prev[1] + k3[1],
            w_prev[2] + k3[2]
        });
        for (auto& val : k4) val *= h;

        std::array<float, 3> w_next;
        for (size_t j = 0; j < 3; ++j) {
            w_next[j] = w_prev[j] + (k1[j] + 2.0f * k2[j] + 2.0f * k3[j] + k4[j]) / 6.0f;
        }

        w.push_back(w_next);
        t_values.push_back(t);
    }

    return {t_values, w};
}
