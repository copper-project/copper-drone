import cv2
from cv2 import aruco

# Define board parameters
squaresX, squaresY = 7, 9
squareLength, markerLength = 28, 22 
dictionary = aruco.getPredefinedDictionary(aruco.DICT_4X4_50)

# Create the Charuco board correctly (New OpenCV API)
charuco_board = aruco.CharucoBoard((squaresX, squaresY), squareLength, markerLength, dictionary)

# Generate the board image
board_img = charuco_board.generateImage((squaresX*squareLength*10, squaresY*squareLength*10))
cv2.imwrite("charuco_board.png", board_img)

print("Charuco board generated successfully!")

