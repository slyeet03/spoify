import os
import spotipy
import re
from spotipy.oauth2 import SpotifyClientCredentials
from dotenv import load_dotenv

# Load environment variables from .env file
load_dotenv()

# spotify cred
SPOTIPY_CLIENT_ID = os.environ.get('SPOTIPY_CLIENT_ID')
SPOTIPY_CLIENT_SECRET = os.environ.get('SPOTIPY_CLIENT_SECRET')
SPOTIPY_REDIRECT_URI = "http://localhost:8888/callback"

def authenticate():
	'''
	Authenticate the spotify api
	Returns:
		sp: Spotify object
	'''
	client_credentials_manager = SpotifyClientCredentials(client_id=SPOTIPY_CLIENT_ID, client_secret=SPOTIPY_CLIENT_SECRET)
	sp = spotipy.Spotify(client_credentials_manager=client_credentials_manager)
	return sp

def fetch_tracks_from_playlist(playlist_url, sp):
	'''
	Extract the playlist info
	Args:
		url: Spotify link
		sp: Spotify object
	Returns:
		List of info of each track
	'''
	playlist_id = re.search(r'/playlist/(\w+)', playlist_url).group(1)
	results = sp.playlist_tracks(playlist_id)
	return results['items'] if 'items' in results else []

def fetch_tracks_from_album(album_url, sp):
	'''
	Extract the album info
	Args:
		url: Spotify link
		sp: Spotify object
	Returns:
		List of info of each track
	'''
	album_id = album_url.split('/')[-1].split('?')[0]
	results = sp.album_tracks(album_id)
	return results['items'] if 'items' in results else []

def get_tracks(url, sp):
	'''
	Extract the track info from playlist/album depending upon the link
	Args:
		url: Spotify link
		sp: Spotify object
	Returns:
		List of info of each track
	'''
	if re.match(r'.*spotify.com/playlist/.*', url):
		return fetch_tracks_from_playlist(url, sp)
	elif re.match(r'.*spotify.com/album/.*', url):
		return fetch_tracks_from_album(url, sp)
	else:
		raise ValueError("Invalid URL. Please enter a valid Spotify playlist or album URL.")

def spotify_query(url):
	'''
	Set up a query to search on youtube for the respective spotify link
	Args:
		url: Spotify link of playlist/album
	Returns:
		queries: list of query to be searched on youtube
	'''
	sp = authenticate()
	queries = []
	try:
		tracks = get_tracks(url, sp)
		#identifying service
		if re.match(r'.*spotify.com/playlist/.*', url):
			type_ = "playlist"
		elif re.match(r'.*spotify.com/album/.*', url):
			type_ = "album"

		for track in tracks:
			if type_ == "playlist": 
				track_name = track["track"]["name"]
				artist_uri = track["track"]["artists"][0]["uri"]
				artist_info = sp.artist(artist_uri)
				artist_name = artist_info["name"]
			elif type_ == "album":
				track_name = track["name"]
				artist_name = track["artists"][0]["name"]

			query = f"{track_name} {artist_name} official song"
			queries.append(query)

	except ValueError as e:
		print(e)

	return queries



