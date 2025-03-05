import json
import re
import structures
import os


def process_json_file(name, album_id, artist_id):
    link = get_link(name)

    file_json = open(link, "r")
    album_json = file_json.read()
    file_json.close()

    return process_json(album_json, album_id, artist_id)


def get_link(name):
    return name.replace(" ", "")+".json"


def process_json(album_json, album_id, artist_id):
    data = json.loads(album_json)
    album_name = data["name"].lower()

    off_cover = data["cover_art_thumbnail_url"]

    new_cover = get_cover_link(artist_id, album_id)
    download_cover(off_cover, new_cover, artist_id)

    if data["release_date_components"]:
        year = data["release_date_components"]["year"]
        month = data["release_date_components"]["month"]
        day = data["release_date_components"]["day"]
        release = f"{year}-{month}-{day}"
    else:
        release = None

    songs = [analyze_song(song) for song in data["tracks"]]
    return structures.album(album_name, new_cover, songs, artist_id, release)


def get_cover_link(artist_id, album_id):
    cover_link = f"covers/{artist_id}/{album_id}.png"
    return cover_link


def download_cover(off_cover, new_cover, artist_id):
    if not os.path.isdir("covers"):
        os.system("mkdir covers")

    if not os.path.isdir(f"covers/{artist_id}"):
        os.system(f"mkdir 'covers/{artist_id}'")

    if not os.path.isfile(new_cover):
        os.system(f"wget {off_cover} -O '{new_cover}'")


def analyze_song(song):
    name = song["song"]["title"]
    name = name.lower()

    lyrics = song["song"]["lyrics"]
    lyrics = format_lyrics(lyrics)

    return structures.song(name, lyrics)


def format_lyrics(lyrics):
    if lyrics != "":
        lyrics = lyrics.split("Lyrics")[1].lstrip()
        lyrics = lyrics.split("Embed")[0].rstrip()
        lyrics = lyrics.replace("You might also like", "")
        lyrics = re.sub(r'See (.*?) LiveGet', 'liveget', lyrics)
        lyrics = lyrics.split("liveget")[0].rstrip()

        lyrics = re.sub(r'[\(\[].*?[\)\]]', '', lyrics)
        lyrics = re.sub("\n{3,}", "\n\n", lyrics)

        lyrics = lyrics.replace("\u2005", " ")

        while lyrics[0] == '\n':
            lyrics = lyrics[1:]

        lyrics = lyrics.lower()

    return lyrics
