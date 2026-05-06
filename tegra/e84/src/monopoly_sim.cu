// monopoly_sim.cu
#include <cuda_runtime.h>
#include <curand_kernel.h>
#include <stdio.h>

#define BOARD_SIZE 40
#define SIMULATIONS_PER_THREAD 1000

__device__ int chance_cards[] = {0, 24, 11, -5, -4, -4, -3, -2,
                                  -1,-1,-1,-1,-1,-1,-1,-1};
__device__ int cc_cards[]     = {0, -2,
                                  -1,-1,-1,-1,-1,-1,-1,-1,
                                  -1,-1,-1,-1,-1,-1};

__device__ int handle_card(int pos, int card) {
    switch(card) {
        case -1: return pos;
        case -2: return 10;
        case -3: return (pos >= 3) ? pos - 3 : BOARD_SIZE + pos - 3;
        case -4:
            if (pos < 5 || pos >= 35) return 5;
            else if (pos < 15)        return 15;
            else if (pos < 25)        return 25;
            else                      return 35;
        case -5:
            return (pos < 12 || pos >= 28) ? 12 : 28;
        default:
            return (card >= 0) ? card : pos;
    }
}

__device__ int process_square(int pos, int* chance_pos, int* cc_pos) {
    if (pos == 7 || pos == 22 || pos == 36) {
        int card = chance_cards[*chance_pos];
        *chance_pos = (*chance_pos + 1) % 16;
        pos = handle_card(pos, card);
    } else if (pos == 2 || pos == 17 || pos == 33) {
        int card = cc_cards[*cc_pos];
        *cc_pos = (*cc_pos + 1) % 16;
        pos = handle_card(pos, card);
    } else if (pos == 30) {
        pos = 10;
    }
    return pos;
}

__global__ void simulate(unsigned long long* global_counts,
                          unsigned long long seed,
                          int total_threads) {
    int tid = blockIdx.x * blockDim.x + threadIdx.x;
    if (tid >= total_threads) return;

    // Each thread gets local counts to avoid atomics in hot loop
    unsigned long long local_counts[BOARD_SIZE] = {0};

    curandState state;
    curand_init(seed, tid, 0, &state);

    int pos        = 0;
    int chance_pos = 0;
    int cc_pos     = 0;

    for (int i = 0; i < SIMULATIONS_PER_THREAD; i++) {
        // Roll two 4-sided dice (1-4 each)
        int roll = (curand(&state) % 4 + 1) + (curand(&state) % 4 + 1);
        pos = (pos + roll) % BOARD_SIZE;
        pos = process_square(pos, &chance_pos, &cc_pos);
        local_counts[pos]++;
    }

    // Accumulate into global counts with atomics
    for (int s = 0; s < BOARD_SIZE; s++) {
        if (local_counts[s] > 0) {
            atomicAdd(&global_counts[s], local_counts[s]);
        }
    }
}

// C-callable entry point for Rust FFI
extern "C" void run_simulation(unsigned long long* counts_out) {
    const int THREADS_PER_BLOCK = 256;
    const int NUM_BLOCKS        = 256;   // 65536 total threads
    const int TOTAL_THREADS     = THREADS_PER_BLOCK * NUM_BLOCKS;

    unsigned long long* d_counts;
    cudaMalloc(&d_counts, BOARD_SIZE * sizeof(unsigned long long));
    cudaMemset(d_counts, 0, BOARD_SIZE * sizeof(unsigned long long));

    simulate<<<NUM_BLOCKS, THREADS_PER_BLOCK>>>(
        d_counts,
        (unsigned long long)time(NULL),
        TOTAL_THREADS
    );
    cudaDeviceSynchronize();

    cudaMemcpy(counts_out,
               d_counts,
               BOARD_SIZE * sizeof(unsigned long long),
               cudaMemcpyDeviceToHost);
    cudaFree(d_counts);
}