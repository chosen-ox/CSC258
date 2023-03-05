#include <stdlib.h>
#include <omp.h>
#include <iostream>
#include <iomanip>
#include <cmath>
#include <stdlib.h>
#include <time.h>
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



void omp_matmul(const float* A, const float* B, float* C, int nsize, int thread_num) {
    int i, j, k;

    #pragma omp parallel for private(i, j, k) shared(A, B, C, nsize) num_threads(thread_num)
    for (i = 0; i < nsize; i++) {
        for (j = 0; j < nsize; j++) {
            C[i * nsize + j] = 0;
            for (k = 0; k < nsize; k++) {
                C[i * nsize + j] += A[i * nsize + k] * B[k * nsize + j];
            }
        }
    }
}
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
    omp_matmul(A, B, D, nsize, 1);
    clock_gettime(CLOCK_REALTIME, &seq_stop);
    std::cout << "OMP Time: " << ((seq_stop.tv_sec - seq_start.tv_sec) + (seq_stop.tv_nsec - seq_start.tv_nsec) / 1E9) << '\n';
    std::cout << "OMP speed 1 thread:" << ( (nsize * nsize)/ ((seq_stop.tv_sec - seq_start.tv_sec) + (seq_stop.tv_nsec - seq_start.tv_nsec) / 1E9)  * nsize * 2 / 1E6 )<< '\n';


    clock_gettime(CLOCK_REALTIME, &seq_start);
    omp_matmul(A, B, D, nsize, 2);
    clock_gettime(CLOCK_REALTIME, &seq_stop);
    std::cout << "OMP Time: " << ((seq_stop.tv_sec - seq_start.tv_sec) + (seq_stop.tv_nsec - seq_start.tv_nsec) / 1E9) << '\n';
    std::cout << "OMP speed 2 thread:" << ( (nsize * nsize)/ ((seq_stop.tv_sec - seq_start.tv_sec) + (seq_stop.tv_nsec - seq_start.tv_nsec) / 1E9)  * nsize * 2 / 1E6 )<< '\n';


    clock_gettime(CLOCK_REALTIME, &seq_start);
    omp_matmul(A, B, D, nsize, 4);
    clock_gettime(CLOCK_REALTIME, &seq_stop);
    std::cout << "OMP Time: " << ((seq_stop.tv_sec - seq_start.tv_sec) + (seq_stop.tv_nsec - seq_start.tv_nsec) / 1E9) << '\n';
    std::cout << "OMP speed 4 thread:" << ( (nsize * nsize)/ ((seq_stop.tv_sec - seq_start.tv_sec) + (seq_stop.tv_nsec - seq_start.tv_nsec) / 1E9)  * nsize * 2 / 1E6 )<< '\n';

    clock_gettime(CLOCK_REALTIME, &seq_start);
    omp_matmul(A, B, D, nsize, 8);
    clock_gettime(CLOCK_REALTIME, &seq_stop);
    std::cout << "OMP Time: " << ((seq_stop.tv_sec - seq_start.tv_sec) + (seq_stop.tv_nsec - seq_start.tv_nsec) / 1E9) << '\n';
    std::cout << "OMP speed 8 thread:" << ( (nsize * nsize)/ ((seq_stop.tv_sec - seq_start.tv_sec) + (seq_stop.tv_nsec - seq_start.tv_nsec) / 1E9)  * nsize * 2 / 1E6 )<< '\n';

    clock_gettime(CLOCK_REALTIME, &seq_start);
    seq_matmul(A, B, C, nsize);
    clock_gettime(CLOCK_REALTIME, &seq_stop);
    std::cout << "Seq (vectorized) Time: " << ((seq_stop.tv_sec - seq_start.tv_sec) + (seq_stop.tv_nsec - seq_start.tv_nsec) / 1E9) << '\n';

    verify(C, D, nsize);


    free(A);
    free(B);
    free(C);
    return 0;
}