import re
import os
from youtubesearchpython import VideosSearch
import time

def spotify_to_youtube(query):
	'''
	To find the youtube link for the specified query
	Args:
		query: name of track and artist
	Returns:
		youtube_link: Link for the specified query
	'''
	try:
		search_query = query
		videos_search = VideosSearch(search_query, limit=1)
		# Retrieving search results
		results = videos_search.result()
		if results and len(results['result']) > 0:
			first_result = results['result'][0] # fecthing the first result
			title = first_result['title']
			youtube_link = first_result['link']
			print("Title:", title)
			return youtube_link
		else:
			print("No YouTube link found for the song.")
  
	except Exception as e:
		print(f'An unexpected error occurred: {str(e)}')