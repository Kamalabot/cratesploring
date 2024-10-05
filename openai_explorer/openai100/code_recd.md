import cv2
import numpy as np

# Input image
img = cv2.imread(\"object.png\")

# Known object size in centimeters
object_size = 10 

# Generate object points for a cube with known size
objpoints = np.array([
                     [0, 0, 0],
                     [object_size, 0, 0],
                     [0, object_size, 0],
                     [object_size, object_size, 0],
                     [0, 0, -object_size],
                     [object_size, 0, -object_size],
                     [0, object_size, -object_size],
                     [object_size, object_size, -object_size]
                     ], dtype=np.float32)

# Generate image points by detecting corners of the object in the input image using OpenCV's corner detection function
gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)
ret, corners = cv2.findChessboardCorners(gray, (2,2), None)
imgpoints = corners.reshape(-1,2)

# Calculate camera matrix and distortion coefficients using object and image points
ret, mtx, dist, rvecs, tvecs = cv2.calibrateCamera([objpoints], [imgpoints], gray.shape[::-1], None, None)

# Calculate rotation and translation vectors of the object using solvePnP function
retval, rvec, tvec = cv2.solvePnP(objpoints, imgpoints, mtx, dist)

# Project object points onto the image plane to get predicted image points
imgpts, jac = cv2.projectPoints(objpoints, rvec, tvec, mtx, dist)

# Calculate the distance between two opposite corners of the object in the image
dist_x = abs(imgpts[3][0][0] - imgpts[0][0][0])
dist_y = abs(imgpts[3][0][1] - imgpts[2][0][1])

# Convert pixel distances to real-world distances in centimeters using the known object size
object_dimensions = (dist_x*(object_size/imgpts[3][0][0]), dist_y*(object_size/imgpts[3][0][0]))

# Print the dimensions of the object
print('Length: {} cm'.format(object_dimensions[0]))
print('Width: {} cm'.format(object_dimensions[1]))

