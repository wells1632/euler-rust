// src/prime_search.cu
#include <cuda_runtime.h>
#include <stdint.h>
#include <stdio.h>

#define CUDA_CHECK(call) \
    do { \
        cudaError_t err = (call); \
        if (err != cudaSuccess) { \
            printf("CUDA error at %s:%d — %s\n", __FILE__, __LINE__, cudaGetErrorString(err)); \
        } \
    } while(0)

__device__ bool is_prime_gpu(uint64_t n) {
    if (n < 2) return false;
    if (n == 2) return true;
    if (n % 2 == 0) return false;
    uint64_t sqrt_n = (uint64_t)sqrtf((float)n);
    for (uint64_t i = 3; i <= sqrt_n; i += 2) {
        if (n % i == 0) return false;
    }
    return true;
}

__device__ uint64_t concatenate_gpu(uint64_t a, uint64_t b) {
    uint64_t tmp = b;
    uint64_t shift = 1;
    if (tmp == 0) shift = 10;
    else while (tmp > 0) { shift *= 10; tmp /= 10; }
    return a * shift + b;
}

__device__ bool pair_compatible_gpu(uint64_t a, uint64_t b) {
    return is_prime_gpu(concatenate_gpu(a, b)) &&
           is_prime_gpu(concatenate_gpu(b, a));
}

// Each thread handles one (i, j) pair and searches k/l/m within that
__global__ void search_kernel(
    const uint64_t* primes,
    int n_primes,
    uint64_t* result,
    int* found_flag,
    int i_fixed,
    int j_start,
    int j_end
) {
    int j = blockIdx.x * blockDim.x + threadIdx.x + j_start;
    if (j >= j_end || j <= i_fixed || *found_flag) return;
    if (!pair_compatible_gpu(primes[i_fixed], primes[j])) return;

    for (int k = j+1; k < n_primes; k++) {
        if (*found_flag) return;
        if (!pair_compatible_gpu(primes[i_fixed], primes[k])) continue;
        if (!pair_compatible_gpu(primes[j],       primes[k])) continue;
        for (int l = k+1; l < n_primes; l++) {
            if (*found_flag) return;
            if (!pair_compatible_gpu(primes[i_fixed], primes[l])) continue;
            if (!pair_compatible_gpu(primes[j],       primes[l])) continue;
            if (!pair_compatible_gpu(primes[k],       primes[l])) continue;
            for (int m = l+1; m < n_primes; m++) {
                if (*found_flag) return;
                if (!pair_compatible_gpu(primes[i_fixed], primes[m])) continue;
                if (!pair_compatible_gpu(primes[j],       primes[m])) continue;
                if (!pair_compatible_gpu(primes[k],       primes[m])) continue;
                if (!pair_compatible_gpu(primes[l],       primes[m])) continue;
                if (atomicCAS(found_flag, 0, 1) == 0) {
                    result[0] = primes[i_fixed]; result[1] = primes[j];
                    result[2] = primes[k];       result[3] = primes[l];
                    result[4] = primes[m];
                    result[5] = primes[i_fixed]+primes[j]+primes[k]+primes[l]+primes[m];
                }
                return;
            }
        }
    }
}

extern "C" void launch_search(
    const uint64_t* h_primes,
    int n_primes,
    uint64_t* h_result
) {
    uint64_t *d_primes, *d_result;
    int *d_found;

    CUDA_CHECK(cudaMalloc(&d_primes, n_primes * sizeof(uint64_t)));
    CUDA_CHECK(cudaMalloc(&d_result, 6 * sizeof(uint64_t)));
    CUDA_CHECK(cudaMalloc(&d_found,  sizeof(int)));

    CUDA_CHECK(cudaMemcpy(d_primes, h_primes, n_primes * sizeof(uint64_t), cudaMemcpyHostToDevice));
    CUDA_CHECK(cudaMemset(d_result, 0, 6 * sizeof(uint64_t)));
    CUDA_CHECK(cudaMemset(d_found,  0, sizeof(int)));

    int threads = 128;
    // Each kernel launch handles one i value, with j split into batches of j_batch
    int j_batch = 20;

    for (int i = 0; i < n_primes; i++) {
        int found = 0;
        CUDA_CHECK(cudaMemcpy(&found, d_found, sizeof(int), cudaMemcpyDeviceToHost));
        if (found) break;

        for (int j_start = i+1; j_start < n_primes; j_start += j_batch) {
            CUDA_CHECK(cudaMemcpy(&found, d_found, sizeof(int), cudaMemcpyDeviceToHost));
            if (found) break;

            int j_end = j_start + j_batch;
            if (j_end > n_primes) j_end = n_primes;

            int range = j_end - j_start;
            int blocks = (range + threads - 1) / threads;

            search_kernel<<<blocks, threads>>>(
                d_primes, n_primes, d_result, d_found,
                i, j_start, j_end
            );
            CUDA_CHECK(cudaDeviceSynchronize());
        }
    }

    int found = 0;
    CUDA_CHECK(cudaMemcpy(h_result, d_result, 6 * sizeof(uint64_t), cudaMemcpyDeviceToHost));
    CUDA_CHECK(cudaMemcpy(&found, d_found, sizeof(int), cudaMemcpyDeviceToHost));
    h_result[6] = (uint64_t)found;

    cudaFree(d_primes);
    cudaFree(d_result);
    cudaFree(d_found);
}