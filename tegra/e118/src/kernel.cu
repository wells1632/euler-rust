// kernel.cu
#include <stdint.h>
#include <cuda_runtime.h>

__device__ bool is_prime(uint32_t n) {
    if (n < 2) return false;
    if (n == 2) return true;
    if (n % 2 == 0) return false;
    for (uint32_t i = 3; (i * i) <= n; i += 2)
        if (n % i == 0) return false;
    return true;
}

// Each thread processes one permutation
// perms: flattened array of permutations (each 9 digits)
// out:   output buffer — packed prime sets (up to MAX_PARTS partitions per perm)
// out_counts: how many valid partitions each thread found
#define PERM_LEN 9
#define MAX_PARTS 64
#define MAX_PART_NUMS 9

__global__ void find_prime_partitions(
    const uint8_t* perms,
    uint32_t num_perms,
    uint32_t* out,           // [num_perms * MAX_PARTS * MAX_PART_NUMS]
    uint8_t*  out_part_lens, // [num_perms * MAX_PARTS]
    uint32_t* out_counts     // [num_perms]
) {
    uint32_t tid = blockIdx.x * blockDim.x + threadIdx.x;
    if (tid >= num_perms) return;

    const uint8_t* digits = perms + tid * PERM_LEN;
    uint32_t count = 0;

    // Iterative DFS via explicit stack
    // Stack frame: (start, current_len, current[MAX_PART_NUMS], num_value_so_far, end)
    struct Frame {
        uint8_t start;
        uint8_t cur_len;
        uint32_t current[MAX_PART_NUMS];
        uint8_t end;        // current 'end' index in the for-loop
        uint32_t num;       // accumulated number up to 'end'
    };

    Frame stack[MAX_PART_NUMS * PERM_LEN];
    int sp = 0;

    // Seed the initial frame
    stack[0] = {0, 0, {}, 0, 0};
    sp = 1;

    while (sp > 0) {
        Frame& f = stack[sp - 1];

        if (f.start == PERM_LEN) {
            // Valid complete partition
            if (count < MAX_PARTS) {
                uint32_t base = tid * MAX_PARTS * MAX_PART_NUMS + count * MAX_PART_NUMS;
                for (uint8_t k = 0; k < f.cur_len; k++)
                    out[base + k] = f.current[k];
                out_part_lens[tid * MAX_PARTS + count] = f.cur_len;
                count++;
            }
            sp--;
            continue;
        }

        bool pushed = false;
        while (f.end < PERM_LEN) {
            f.num = f.num * 10 + digits[f.end];
            f.end++;
            if (is_prime(f.num)) {
                // Push new frame for recursion
                Frame next;
                next.start   = f.end;
                next.cur_len = f.cur_len + 1;
                for (uint8_t k = 0; k < f.cur_len; k++)
                    next.current[k] = f.current[k];
                next.current[f.cur_len] = f.num;
                next.end = next.start;
                next.num = 0;
                stack[sp++] = next;
                pushed = true;
                break;
            }
        }
        if (!pushed) sp--;
    }

    out_counts[tid] = count;
}

extern "C" void launch_find_prime_partitions(
    const uint8_t* perms,
    uint32_t num_perms,
    uint32_t* out,
    uint8_t*  out_part_lens,
    uint32_t* out_counts
) {
    uint8_t  *d_perms;
    uint32_t *d_out, *d_counts;
    uint8_t  *d_part_lens;

    size_t perms_size     = num_perms * 9 * sizeof(uint8_t);
    size_t out_size       = num_perms * 64 * 9 * sizeof(uint32_t);
    size_t part_lens_size = num_perms * 64 * sizeof(uint8_t);
    size_t counts_size    = num_perms * sizeof(uint32_t);

    cudaMalloc(&d_perms,     perms_size);
    cudaMalloc(&d_out,       out_size);
    cudaMalloc(&d_part_lens, part_lens_size);
    cudaMalloc(&d_counts,    counts_size);

    cudaMemcpy(d_perms, perms, perms_size, cudaMemcpyHostToDevice);

    int threads_per_block = 128;
    int blocks = (num_perms + threads_per_block - 1) / threads_per_block;
    find_prime_partitions<<<blocks, threads_per_block>>>(
        d_perms, num_perms, d_out, d_part_lens, d_counts
    );
    cudaDeviceSynchronize();

    cudaMemcpy(out,           d_out,       out_size,       cudaMemcpyDeviceToHost);
    cudaMemcpy(out_part_lens, d_part_lens, part_lens_size, cudaMemcpyDeviceToHost);
    cudaMemcpy(out_counts,    d_counts,    counts_size,    cudaMemcpyDeviceToHost);

    cudaFree(d_perms);
    cudaFree(d_out);
    cudaFree(d_part_lens);
    cudaFree(d_counts);
}