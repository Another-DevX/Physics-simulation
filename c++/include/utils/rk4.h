#ifndef RK4_H
#define RK4_H

#include <vector>
#include <array>
#include <functional>

std::pair<std::vector<float>, std::vector<std::array<float, 3>>> rk4(
    float a, float b, const std::array<float, 3>& alpha,
    std::function<std::array<float, 3>(float, const std::array<float, 3>&)> f, 
    unsigned int n);

#endif
