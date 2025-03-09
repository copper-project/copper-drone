gst-launch-1.0 -e v4l2src device=/dev/video9 ! image/jpeg,width=1920,height=1080 ! rtpjpegpay ! udpsink host=192.168.1.181 port=5000

