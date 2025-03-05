class song:
    def __init__(self, name, lyrics):
        self.name = name
        self.lyrics = lyrics


class album:
    def __init__(self, name, cover, songs, artist_id, release = None):
        self.name = name
        self.cover = cover
        self.songs = songs
        self.artist = artist_id
        self.release = release
