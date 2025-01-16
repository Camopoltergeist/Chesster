# Tavoitteet
## Graafinen käyttöliittymä
- Toteutetaan käyttäen [raylib-kirjastoa](https://www.raylib.com)
- Ikkuna
- Board
  - 8x8 lauta
  - Nappulat
    - Tekstuurit
  - Koordinaatistomerkit laudan reunassa (A-H ja 1-8)
  - Pelilaudan kääntäminen (jotta voi pelata kumpaakin väriä helposti)
  - Nappuloiden valitseminen
    - Klikkaamalla
	  - Klikkaamalla toista nappulaa valitsee uuden nappulan.
	  - Vain yksi nappula valittuna kerralla.
	  - Klikkaamalla ruutua, jossa ei ole laillista siirtoa, poistaa valinnan.
    - Valitun napin korostaminen jotenkin (outline? ruudun värin muuttaminen?)
    - Laillisten siirtojen näyttäminen valitulle napille
  - Nappuloiden liikuttaminen
    - Kun nappula on valittu, klikkaamalla ruutua, jossa on laillinen siirto, siirtää nappulan siihen.
  - Debug featuret:
	- Kaikkien laillisten siirtojen listaaminen (ja visualisointi jos jaksaa)
    - Botin laskeman siirron näyttäminen (ilman, että se pelataan heti)
	- Bitboardien visualisointi
	- Board editor, jotta pystytään nopeasti testaamaan tiettyjä tilanteita

## Board Representation
- Toteutetaan käyttäen pääasiassa bitboardeja

### Säännöt
- Nappuloiden liikunta
  - Eivät saa liikkua nappuloiden läpi, paitsi knight.
    - Syödessään vastustajan nappuloita saavat korvata syödyn nappulan paikan.
  - Syövät vastustajan nappulat, jos laskeutuvat niiden ruudulle, paitsi pawn.
  - Mikään siirto ei saa jättää kingiä uhatuksi.
  - Pawn
    - Liikkuu eteenpäin yksi ruutu kerrallaan
	- Saa liikkua kaksi ruutua eteenpäin aloituspaikasta
	- Syö yhden ruudun etuviistoon kulkusuuntaan katsottuna, ei syö suoraan eteenpäin
	- En passant -sääntö
	- Ylenee päästessään viimeiselle riville
	  - Queen
	  - Rook
	  - Bishop
	  - Knight
	- 1 material
  - King
    - Liikkuu yhden ruudun mihin tahansa suuntaan.
	- Castling -sääntö
  - Queen
    - Liikkuu 8:aan eri suuntaan niin pitkälle kun on tilaa.
	- 9 material
  - Bishop
    - Liikkuu viistosti niin pitkälle kun on tilaa.
	- 2 material
  - Rook
    - Liikkuu pysty- tai vaakasuoraan niin pitkälle kun on tilaa.
	- Castling -sääntö
	- 5 material
  - Knight
    - Liikkuu L:n muotoisissa kuvioissa. Eli 1 ruutu pystysuunnassa ja 2 vaakasuunnassa tai toisin päin.
	- Saa hypätä nappuloiden yli, vain laskeutumisruudun tarvitsee olla tyhjä (tai sisältää vihollisen nappula)
	- 2 material
- Check
  - Jos jokin vastapelaajan nappula uhkaa syödä kuninkaan, ainoat lailliset siirrot ovat siirrot, jotka poistavat uhan.
- Checkmate
  - Jos kuningas on uhattuna, ja mikään siirto ei pysty poistamaan uhkaa, pelaaja, joka uhkaa kuningasta voittaa.
- Stalemate
  - Jos kuningas ei ole uhattuna, ja vuorossa olevalla pelaajalla ei ole yhtään laillista siirtoa jäljellä, peli päättyy tasapeliin.
- Threefold Repetition rule
  - Jos pelissä toistuu täysin sama lautakokoonpano (kaikki nappulat ovat samoissa paikoissa) kolmella eri vuorolla, vuorossa oleva pelaaja voi lopettaa pelin, jolloin peli loppuu tasapeliin.
  - Tätä sääntöä ei välttämättä tarvitse implementoida.

### Muut Implementaation liittyvät asiat
- Tulee jossain vaiheessa

## Siirron laskenta -algoritmi
- Tulee jossain vaiheessa