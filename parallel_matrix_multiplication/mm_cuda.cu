/*
    Created by: Andrew Sexton
          Date: March 21, 2022
      Modified: February 20, 2023 by Owen Wacha

    CSC258/458 - Parallel & Distributed Systems.
*/
#include <iostream>
#include <iomanip>
#include <cmath>
#include <stdlib.h>
#include <time.h>

/* Use this macro to catch and print out runtime errors from the GPU */
/* This does not work on kernel functions eg. some_kernel<<<...>>>() */
/* Ex. cudaErrChk(cudaMalloc(...)) */
/*     cudaErrChk(cudaDeviceSynchronize()) */
#define cudaErrchk(ans) { gpuAssert((ans), __FILE__, __LINE__); }
inline void gpuAssert(cudaError_t code, const char* file, int line, bool abort=true) {
    if (code != cudaSuccess) {
        std::cout << "GPUAssert: " << cudaGetErrorString(code) << " " << file << " line " << line << std::endl;
        if (abort) { exit(code); }
    }
}

/* Vectorizable version of matrix multiplication for comparison */
__global__ void seq_matmul(const float* A, const float* B, float* C, int nsize) {
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

// Print a comma-separated 2D array to stdout
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

// GPU Kernel
__global__ void gpu_matmul(float* A, float* B, float* C, int nsize) {
    /* Add your cuda solution code here */
    //
    // int x = blockIdx.x * blockDim.x + threadIdx.x;
    // int y = blockIdx.y * blockDim.y + threadIdx.y;
    // if (x < nsize && y < nsize) {
    // float tmp = 0.0f;
    // for(int k = 0; k < nsize; k++) {
    //     tmp += A[k + (x * nsize)] * B[y + (k * nsize)]; 
    // }

    // C[x*nsize + y] = tmp; 
    // }
    //

    //
int row = blockIdx.y * blockDim.y + threadIdx.y; 
    int col = blockIdx.x * blockDim.x + threadIdx.x;
    float sum = 0;
    if( col < nsize && row < nsize) 
    {
        for(int i = 0; i < nsize; i++) 
        {
            sum += A[row * nsize + i] * B[i * nsize + col];
        }
        C[row * nsize + col] = sum;
    }

}


int main(int argc, char *argv[]) {
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

    float * A1;
    float * B1;
    float * C1;
    float * D1;
    size_t bytes = nsize * nsize * sizeof(float);

    // cudaMalloc(&A1, bytes);
    // cudaMalloc(&B1, bytes);
    // cudaMalloc(&C1, bytes);

        cudaErrchk(cudaMalloc(&A1, bytes));
        cudaErrchk(cudaMalloc(&B1, bytes));
        cudaErrchk(cudaMalloc(&C1, bytes));
        cudaErrchk(cudaMalloc(&D1, bytes));


    // Fill CPU side arrays
    for(int i = 0; i < nsize; i++) {
        for(int j = 0; j < nsize; j++) {
            int idx = static_cast<float>(j + (i * nsize));
            A[idx] = idx + 1.0f;
            B[idx] = 1.0f / (idx + 1.0f);
        }
    }

    cudaErrchk(cudaMemcpy( A1, A, bytes, cudaMemcpyHostToDevice));
    cudaErrchk(cudaMemcpy( B1, B, bytes, cudaMemcpyHostToDevice));
    // Start GPU timer

    /* Add your code here */
    //
    //
    //
    /*===================*/
    dim3 threadsPerBlock(16, 16);
    dim3 numBlocks(nsize/ threadsPerBlock.x + 1, nsize / threadsPerBlock.y + 1);
    clock_gettime(CLOCK_REALTIME, &gpu_start);
    gpu_matmul<<<numBlocks, threadsPerBlock>>>(A1, B1, C1, nsize);

    // gpu_matmul<<<numBlocks, threadsPerBlock>>>(A1, B1, C1, nsize);

        // cudaErrchk(cudaMemcpy(D, C1, bytes, cudaMemcpyDeviceToHost ));
        // gpuErrchk(cudaFree(DadC1,
        // gpuErrchk(cudaFree( bd ));
    // Stop GPU timer
    //clock_gettime(CLOCK_REALTIME, &gpu_stop);	
    cudaErrchk( cudaPeekAtLastError() );
    cudaErrchk( cudaDeviceSynchronize() );
    cudaErrchk(cudaMemcpy( C, C1, bytes, cudaMemcpyDeviceToHost));
    clock_gettime(CLOCK_REALTIME, &gpu_stop); 
    // cudaErrchk(cudaFree(A1));
    // cudaErrchk(cudaFree(B1));
    // cudaErrchk(cudaFree(C1));
    //print_array(C, nsize);
    std::cout << "GPU Time: " << ((gpu_stop.tv_sec - gpu_start.tv_sec) + (gpu_stop.tv_nsec - gpu_start.tv_nsec) / 1E9) << '\n';
    std::cout << "CUDA speed:" << ( (nsize * nsize)/ ((gpu_stop.tv_sec - gpu_stop.tv_sec) + (gpu_stop.tv_nsec - gpu_start.tv_nsec) / 1E9)  * nsize * 2 / 1E6 )<< '\n';

    // Compute Vectorized version
    // Modifies array C in place.
    clock_gettime(CLOCK_REALTIME, &seq_start);
    seq_matmul<<<1, 1>>>(A1, B1, D1, nsize);
    cudaErrchk( cudaPeekAtLastError() );
    cudaErrchk( cudaDeviceSynchronize() );
    // seq_matmul(A, B, C, nsize);
    clock_gettime(CLOCK_REALTIME, &seq_stop);
    printf("111\n");
    std::cout << "Seq (vectorized) Time: " << ((seq_stop.tv_sec - seq_start.tv_sec) + (seq_stop.tv_nsec - seq_start.tv_nsec) / 1E9) << '\n';
    cudaErrchk(cudaMemcpy( D, D1, bytes, cudaMemcpyDeviceToHost));
    // print_array(C, nsize);

    /* Add Verification Step Here */
    // ...
    // verify(C, D, nsize);

    // Clean up
    delete[] A;
    delete[] B;
    delete[] C;
    delete[] D;
    cudaErrchk(cudaFree(A1));
    cudaErrchk(cudaFree(B1));
    cudaErrchk(cudaFree(C1));
    cudaErrchk(cudaFree(D1));

    return 0;
}
