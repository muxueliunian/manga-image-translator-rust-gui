#include "densecrf.h"

typedef Eigen::Matrix<float, Eigen::Dynamic, Eigen::Dynamic, Eigen::RowMajor> NumpyMatF;

static Eigen::MatrixXf buf2matf(const float* mem, size_t h, size_t w) {
    return Eigen::Map<const NumpyMatF>(mem, h, w);
}

static void matf2buf(const Eigen::MatrixXf& mat, float* mem) {
    Eigen::Map<NumpyMatF>(mem, mat.rows(), mat.cols()) = mat;
}

extern "C" void run_densecrf(
    const float* unary,
    int width,
    int height,
    int n_classes,
    const unsigned char* image,
    int num_iterations,
    float* out_probs)
{
    Eigen::MatrixXf unary_mat = buf2matf(unary, n_classes, width * height);

    DenseCRF2D d(width, height, n_classes);
    d.setUnaryEnergy(unary_mat);
    auto* gaussian_compat = new PottsCompatibility(3);
    auto* bilateral_compat = new PottsCompatibility(20);
    d.addPairwiseGaussian(1, 1,gaussian_compat, DIAG_KERNEL, NO_NORMALIZATION);
    d.addPairwiseBilateral(23, 23, 7, 7, 7, image, bilateral_compat, DIAG_KERNEL, NO_NORMALIZATION);

    Eigen::MatrixXf result = d.inference(num_iterations);
    matf2buf(result, out_probs);
}
