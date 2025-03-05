import mysql.connector
import getpass


def get_database_password():
    return getpass.getpass("Insert database password: ")


connector = mysql.connector.connect(
    host="localhost",
    user="balalaika_user",
    password=get_database_password(),
    database="balalaika",
)

cursor = connector.cursor()


def process_albums(album_list):
    [process_album(album, album_id)
     for album_id, album in enumerate(album_list, 1)]


def process_album(album, album_id):
    upload_album(album, album_id)
    [upload_song(song, album_id) for song in album.songs]


def upload_album(album, album_id):
    album.name = album.name.lower()
    cursor.execute("""
        INSERT INTO album (
            name, cover, artist_id
        )
        VALUES (
            %(name)s, %(cover)s, %(artist_id)s
        );
    """, {
        'name': album.name,
        'cover': album.cover,
        'artist_id': album.artist,
    })

    if album.release:
        cursor.execute("""
            UPDATE album
            SET release_date = (%(release)s)
            WHERE id = %(album_id)s;
        """, {
            'release': album.release,
            'album_id': album_id,
        })


def upload_song(song, album_id):
    cursor.execute("""
        INSERT INTO song (
            name, lyrics, album_id
        )
        VALUES (
            %(name)s, %(lyrics)s, %(album_id)s
        )
    """, {
        'name': song.name,
        'lyrics': song.lyrics,
        'album_id': album_id
    })


def process_artists(artist_names):
    [process_artist(artist) for artist in artist_names]


def process_artist(artist):
    artist = artist.lower()
    cursor.execute("""
        INSERT INTO artist (
            name
        )
        VALUES (
            %(name)s
        )
    """, {
        'name': artist,
    })


def close():
    cursor.close()
    connector.commit()
    connector.close()
