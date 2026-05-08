#include <stdint.h>

__device__ bool reaches_89_device(uint32_t n) {
    while (n != 1 && n != 89) {
        uint32_t sum = 0;
        while (n > 0) {
            uint32_t digit = n % 10;
            sum += digit * digit;
            n /= 10;
        }
        n = sum;
    }
    return n == 89;
}

__global__ void check_numbers(uint32_t start, uint32_t limit, uint32_t* result) {
    uint32_t idx = blockIdx.x * blockDim.x + threadIdx.x;
    uint32_t n = start + idx;
    if (n < limit && reaches_89_device(n)) {
        atomicAdd(result, 1u);
    }
}

extern "C" uint32_t count_reaching_89_cuda(uint32_t limit) {
    uint32_t* d_result;
    uint32_t h_result = 0;

    cudaMalloc(&d_result, sizeof(uint32_t));
    cudaMemcpy(d_result, &h_result, sizeof(uint32_t), cudaMemcpyHostToDevice);

    int threads = 256;
    int blocks = (limit + threads - 1) / threads;
    check_numbers<<<blocks, threads>>>(1, limit, d_result);

    cudaMemcpy(&h_result, d_result, sizeof(uint32_t), cudaMemcpyDeviceToHost);
    cudaFree(d_result);

    return h_result;
}