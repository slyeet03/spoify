import spotify_track_fetcher
import yt_download
import spotify_to_yt_link

def menu(url):
	output_path = "C:/Users/chitr/Desktop/yt audio" 
	queries = spotify_track_fetcher.spotify_query(url)
	
	for query in queries:
		yt_link = spotify_to_yt_link.spotify_to_youtube(query)
		yt_download.audio(yt_link, output_path)

