#!sh
ffmpeg -video_size 800x660 -framerate 15 -f x11grab -i :1.0+0,0 output.mp4
ffmpeg -i output.mp4 -c:v libx264 -crf 20 -preset slow -vf format=yuv420p -c:a aac -movflags +faststart output2.mp4
