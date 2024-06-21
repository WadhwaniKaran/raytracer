import sys 
from PIL import Image

if sys.argv[1] == None:
    print("Enter path to image")
else:
    img = Image.open(sys.argv[1])
    img.show()
