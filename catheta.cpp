#include <opencv2/opencv.hpp>
#include <opencv2/calib3d/calib3d.hpp>
#include <opencv2/highgui/highgui.hpp>
#include <opencv2/imgproc/imgproc.hpp>
#include <stdio.h>
#include <iostream>
#include <CMath>
#include "opencv2/imgcodecs.hpp"

// initialize values for StereoSGBM parameters
int numDisparities = 8;
int blockSize = 5;
int preFilterType = 1;
int preFilterSize = 1;
int preFilterCap = 31;
int minDisparity = 0;
int textureThreshold = 10;
int uniquenessRatio = 15;
int speckleRange = 0;
int speckleWindowSize = 0;
int disp12MaxDiff = -1;
int dispType = CV_16S;

// Constants for projectile motion calculations
#define GRAVITY 9.81        // Acceleration due to gravity (m/s^2)
#define PROJECTILE_V 35.0   // Initial velocity of the projectile (m/s)

// Convert angle from radians to degrees
// Formula: degrees = radians * (180/π)
const double radians_to_degrees(radians)
{
    return radians * (M_PI / 180);
}

// Calculate the optimal launch angle for hitting a target at a given distance
// Uses the projectile motion equation: R = (v^2 * sin(2θ)) / g
// where:
// R = range (target_distance)
// v = initial velocity
// θ = launch angle
// g = acceleration due to gravity
//
// Solving for θ gives: θ = (1/2) * arcsin((R*g)/v^2)
// We use the smaller angle solution for a more direct trajectory
const double calc_angle_in_degrees(target_distance, initial_velocity)
{
    // Calculate launch angle in radians using the projectile range equation
    // We divide by 2 because we want the smaller of the two possible angles
    const radians = asin((target_distance * GRAVITY) / Math.pow(initial_velocity, 2)) / 2;
    return radians_to_degrees(radians);
}

void main()
{
    // ... rest of the code remains unchanged ...
} 