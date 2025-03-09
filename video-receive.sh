#!/bin/bash
gst-launch-1.0 -e udpsrc port=5000 ! "application/x-rtp, encoding-name=JPEG" ! rtpjpegdepay ! jpegdec ! videoconvert ! fpsdisplaysink video-sink=autovideosink text-overlay=true sync=false
