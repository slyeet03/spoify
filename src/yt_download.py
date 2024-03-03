from pytube import YouTube
import os
import time

def audio(url, output_path):
	'''
	Download the audio of youtube link provided
	Args:
		url: Youtube link
		output_path: Path for where the audio will be saved
	'''
	try:
		yt = YouTube(url)
		# getting the audio stream
		audio_stream = yt.streams.filter(only_audio=True).first()
		# downloading the audio
		if audio_stream:
			# downloading the audio
			time.sleep(3)
			audio_file = audio_stream.download(output_path)
			# renaming the file to have an mp3 extension
			audio_path = f"{output_path}/{yt.title}.mp3"
			os.rename(audio_file, audio_path)
			print("Status: Audio downloaded successfully\n")
		else:
			print("No audio stream available for the given URL.")

	except Exception as e:
		print(f"An error occurred: {str(e)}")
		