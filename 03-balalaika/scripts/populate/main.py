from albums import album_data
import api
import parser
import database


def start():
    print("downloading data...")
    api.download_albums(album_data)

    print("uploading data...")
    upload_albums(
        get_album_data(),
        get_artist_names()
    )
    database.close()
    print("upload finished!")
    print("remember to move the covers directory once you're done")


def upload_albums(album_data, artist_names):
    database.process_artists(artist_names)
    database.process_albums(album_data)


def get_album_data():
    result = []

    for artist_id, artist in enumerate(album_data, 1):
        album_list = artist[1]

        for album_id, album in enumerate(album_list, 1):
            result.append(parser.process_json_file(album, album_id, artist_id))

    return result


def get_artist_names():
    artist_data = []
    [artist_data.append(
        artist[0]
    ) for artist in album_data]
    return artist_data


start()
