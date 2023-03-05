#include <cilk/cilk.h>
#include <stdlib.h>
#include <iostream>
#include <iomanip>
#include <cmath>
#include <stdlib.h>
#include <time.h>
#include <cstring>

void print_array(const float* arr, int nsize) {
    for(int i = 0; i < nsize; i++) {
        for(int j = 0; j < nsize; j++) {
            std::cout << arr[j + (i * nsize)];

            if(j < nsize) {
                std::cout << ", ";
            }
        }
        std::cout << std::endl;
    }
}



void seq_matmul(const float* A, const float* B, float* C, int nsize) {
    float temp;
    for (int i = 0; i < nsize; i++) {
        for (int j = 0; j < nsize; j++) {
            temp = 0.0f;
            for (int k = 0; k < nsize; k++) {
                temp += A[k + (i * nsize)] * B[j + (k * nsize)];
            }
            C[j + (i * nsize)] = temp;
        }
    }
}

// Function for verifying values between two arrays
// by computing abs(X[i] - Y[i]) < EPSILON
void verify(const float* X, const float* Y, int nsize){
    float EPSILON = 1E-4;
    for(int i = 0; i < nsize; i++) {
        for(int j = 0; j < nsize; j++) {
            int idx = j + (i * nsize);

            if(std::fabs(X[idx] - Y[idx]) > EPSILON) {
                std::cout << std::setprecision(15) << "(" << i << ", " << j << "): " << X[idx] << " != " << Y[idx] << std::endl;
            }
        }
    }
}



void cilk_matmul_base(const float* A, const float* B, float* C, int nsize, int length, int row_a, int col_a, int row_b, int col_b, int row_c, int col_c) {
    for (int i = 0; i < length; i++) {
        for (int j = 0; j < length; j++) {
            for (int k = 0; k < length; k++) {
                C[(row_c + i) * nsize + j +col_c] += A[(row_a + i) * nsize + k + col_a] * B[(row_b + k) * nsize + j + col_b];
            }
        }
    }   
}
void cilk_matmul_recursive(const float* A, const float* B, float* C, int nsize, int length, int row_a, int col_a, int row_b, int col_b, int row_c, int col_c) {
    if (length <= 128) {
        cilk_matmul_base(A, B, C, nsize, length, row_a, col_a, row_b, col_b, row_c, col_c);
    } else {
        // A11 * B11
        cilk_spawn cilk_matmul_recursive(A, B, C, nsize, length / 2, row_a, col_a, row_b, col_b, row_c, col_c);
        
        // A11 * B12
        cilk_spawn cilk_matmul_recursive(A, B, C, nsize, length / 2, row_a, col_a, row_b, col_b + length / 2, row_c, col_c + length / 2);

        // A12 * B21
        cilk_spawn cilk_matmul_recursive(A, B, C, nsize, length / 2, row_a + length / 2, col_a, row_b, col_b, row_c + length / 2, col_c);

        // A12 * B22
        cilk_spawn cilk_matmul_recursive(A, B, C, nsize, length / 2, row_a + length / 2, col_a, row_b, col_b + length / 2, row_c + length / 2, col_c + length / 2);
        
        // Avoid race condition
        cilk_sync;

        // A21 * B11
        cilk_spawn cilk_matmul_recursive(A, B, C, nsize, length / 2, row_a, col_a + length / 2, row_b + length / 2, col_b, row_c, col_c);

        // A21 * B12
        cilk_spawn cilk_matmul_recursive(A, B, C, nsize, length / 2, row_a, col_a + length / 2, row_b + length / 2, col_b + length / 2, row_c, col_c + length / 2);

        // A22 * B21
        cilk_spawn cilk_matmul_recursive(A, B, C, nsize, length / 2, row_a + length / 2, col_a + length / 2, row_b + length / 2, col_b, row_c + length / 2, col_c);

        // A22 * B22
        cilk_matmul_recursive(A, B, C, nsize, length / 2, row_a + length / 2, col_a + length / 2, row_b + length / 2, col_b + length / 2, row_c + length / 2, col_c + length / 2);

    }
}

void cilk_matmul(const float* A, const float* B, float* C, int nsize) {
    std::memset(C, 0, sizeof(float) * nsize * nsize);
    cilk_matmul_recursive(A, B, C, nsize, nsize, 0, 0, 0, 0, 0, 0);
    
}


int main(int argc, char** argv) {
    if(argc < 2) {
        std::cout << "Invalid number of arguments: usage " << argv[0] << " <array size>" << std::endl;
        exit(0);
    }

    // Array size
    int nsize = std::atoi(argv[1]);

    // Timing Stuff
    timespec seq_start, seq_stop;
    timespec gpu_start, gpu_stop;

    // CPU side arrays
    // Arrays are one dimensional, indexing is (i, j) => j + (i * nsize)
    // this gives a single index into the array using two loop variables
    float* A = new float[nsize * nsize]();
    float* B = new float[nsize * nsize]();
    float* C = new float[nsize * nsize]();
    float* D = new float[nsize * nsize]();


    // Fill CPU side arrays
    for(int i = 0; i < nsize; i++) {
        for(int j = 0; j < nsize; j++) {
            int idx = static_cast<float>(j + (i * nsize));
            A[idx] = idx + 1.0f;
            B[idx] = 1.0f / (idx + 1.0f);
        }
    }



    clock_gettime(CLOCK_REALTIME, &seq_start);
    seq_matmul(A, B, D, nsize);
    clock_gettime(CLOCK_REALTIME, &seq_stop);
    std::cout << "Seq (vectorized) Time: " << ((seq_stop.tv_sec - seq_start.tv_sec) + (seq_stop.tv_nsec - seq_start.tv_nsec) / 1E9) << '\n';


    clock_gettime(CLOCK_REALTIME, &seq_start);
    cilk_matmul(A, B, C, nsize);
    clock_gettime(CLOCK_REALTIME, &seq_stop);
    std::cout << "CILK Time: " << ((seq_stop.tv_sec - seq_start.tv_sec) + (seq_stop.tv_nsec - seq_start.tv_nsec) / 1E9) << '\n';
    verify(C, D, nsize);
    free(A);
    free(B);
    free(C);
    return 0;
}