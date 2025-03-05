import lyricsgenius
import parser
import os


genius = lyricsgenius.Genius(
    "0uSA9UFGsiO2WozVmbWPhyhOoVmUNuM3PXRt9rvWhptHBMgSO5CZBxGUMkwet5mv"
)


def download_albums(albums):
    [get_album_json(artist[0], album)
     for artist in albums for album in artist[1]]


def get_album_json(artist_name, album_name):
    link = parser.get_link(album_name)
    if os.path.isfile(link):
        return

    album = genius.search_album(album_name, artist_name)
    album.save_lyrics(f"{link}")
