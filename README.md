# fplr - a CLI for [Fantasy Premier League](https://fantasy.premierleague.com/) in Rust

Browse FPL data comfortably from your terminal.

## Installation

```sh
git clone https://github.com/sakihet/fplr.git
cd fplr
cargo install --path .
```

## Commands

- fixture
- gameweek
- live
- player
- player-summary
- team

## Example

```
> fplr player
ID   Name                 Pos  Team             Cost     Selected Form     Points   News                          
430  Haaland              FWD  Man City         15.0     72.9     4.8      122                                    
256  Muñoz                DEF  Crystal Palace   6.1      26.7     7.0      89                                     
21   Rice                 MID  Arsenal          7.1      21.9     4.2      84                                     
82   Semenyo              MID  Bournemouth      7.6      46.8     1.6      83                                     
260  Guéhi                DEF  Crystal Palace   5.2      35.9     3.8      83                                     
226  Chalobah             DEF  Chelsea          5.2      11.1     6.6      82                                     
5    Gabriel              DEF  Arsenal          6.3      16.3     0.2      81       Thigh injury - Unknown return date
488  Bruno G.             MID  Newcastle        6.9      10.7     7.0      81                                     
136  Thiago               FWD  Brentford        6.9      28.5     6.6      80                                     
8    J.Timber             DEF  Arsenal          6.5      35.2     2.4      78                                     
257  Lacroix              DEF  Crystal Palace   5.1      6.6      5.6      78                                     
414  Foden                MID  Man City         8.5      22.2     9.8      77                                     
295  Keane                DEF  Everton          4.6      2.2      7.0      76                                     
242  Dewsbury-Hall        MID  Everton          5.0      5.6      9.6      74                                     
36   Cash                 DEF  Aston Villa      4.7      7.5      5.4      73                                     
291  Tarkowski            DEF  Everton          5.4      6.4      7.0      73                                     
7    Calafiori            DEF  Arsenal          5.8      15.0     2.2      71       Suspended until 20 Dec        
72   Senesi               DEF  Bournemouth      5.0      19.3     3.0      71       Thigh injury - 75% chance of playing
299  Ndiaye               MID  Everton          6.5      9.8      3.8      71                                     
236  Neto                 MID  Chelsea          7.3      9.2      5.4      70  

> fplr player --position midfielder --team city --sort form
ID   Name                 Pos  Team             Cost     Selected Form     Points   News                          
414  Foden                MID  Man City         8.5      22.2     9.8      77                                     
418  Doku                 MID  Man City         6.6      9.6      5.2      59                                     
417  Cherki               MID  Man City         6.4      3.4      4.8      43                                     
423  N.Gonzalez           MID  Man City         5.9      0.1      3.8      39                                     
416  Bernardo             MID  Man City         6.2      0.5      2.4      32                                     
427  Reijnders            MID  Man City         5.3      14.3     2.4      46                                     
413  Marmoush             MID  Man City         8.3      2.5      0.8      15                                     
415  Savinho              MID  Man City         6.9      0.4      0.8      19                                     
420  Gündoğan             MID  Man City         6.3      0.1      0.0      0        has joined Galatasaray permanently.
421  Rodrigo              MID  Man City         6.3      0.4      0.0      12       Muscle injury - Unknown return date
422  Kovačić              MID  Man City         5.9      0.0      0.0      1        Ankle injury - Unknown return date
424  Bobb                 MID  Man City         5.2      0.2      0.0      18                                     
425  Echeverri            MID  Man City         5.4      0.0      0.0      0        Has joined Bayer Leverkusen on loan for the rest of the season.
428  Nypan                MID  Man City         5.0      0.0      0.0      0        Has joined Middlesbrough on loan for the rest of the season.
429  Phillips             MID  Man City         4.8      0.1      0.0      0                                      
739  Mukasa               MID  Man City         4.4      0.0      0.0      0                                      
742  McAidoo              MID  Man City         4.5      0.0      0.0      0

> fplr player --position defender --sort points
ID   Name                 Pos  Team             Cost     Selected Form     Points   News                          
256  Muñoz                DEF  Crystal Palace   6.1      26.7     7.0      89                                     
260  Guéhi                DEF  Crystal Palace   5.2      35.9     3.8      83                                     
226  Chalobah             DEF  Chelsea          5.2      11.1     6.6      82                                     
5    Gabriel              DEF  Arsenal          6.3      16.3     0.2      81       Thigh injury - Unknown return date
8    J.Timber             DEF  Arsenal          6.5      35.2     2.4      78                                     
257  Lacroix              DEF  Crystal Palace   5.1      6.6      5.6      78                                     
295  Keane                DEF  Everton          4.6      2.2      7.0      76                                     
36   Cash                 DEF  Aston Villa      4.7      7.5      5.4      73                                     
291  Tarkowski            DEF  Everton          5.4      6.4      7.0      73                                     
7    Calafiori            DEF  Arsenal          5.8      15.0     2.2      71       Suspended until 20 Dec        
72   Senesi               DEF  Bournemouth      5.0      19.3     3.0      71       Thigh injury - 75% chance of playing
258  Mitchell             DEF  Crystal Palace   5.0      2.6      4.8      68                                     
261  Richards             DEF  Crystal Palace   4.6      4.2      4.8      68                                     
408  Rúben                DEF  Man City         5.6      5.1      5.6      67                                     
476  Burn                 DEF  Newcastle        5.1      8.3      3.6      66                                     
575  Van de Ven           DEF  Spurs            4.7      30.3     1.4      64                                     
224  Cucurella            DEF  Chelsea          6.2      23.1     4.0      63                                     
411  O’Reilly             DEF  Man City         5.2      7.5      4.4      62                                     
225  James                DEF  Chelsea          5.6      8.3      3.2      61                                     
151  Van Hecke            DEF  Brighton         4.5      3.0      5.0      60

> fplr player --position goalkeeper
ID   Name                 Pos  Team             Cost     Selected Form     Points   News                          
1    Raya                 GKP  Arsenal          6.0      34.1     2.8      66                                     
287  Pickford             GKP  Everton          5.5      10.4     6.2      66                                     
670  Roefs                GKP  Sunderland       4.7      8.1      2.0      64                                     
253  Henderson            GKP  Crystal Palace   5.1      9.0      4.8      60                                     
220  Sánchez              GKP  Chelsea          4.8      13.9     5.2      57                                     
565  Vicario              GKP  Spurs            4.9      8.3      2.2      57                                     
469  Pope                 GKP  Newcastle        5.1      7.2      1.2      55       Groin Injury - Expected back 26 Dec
32   Martinez             GKP  Aston Villa      5.1      3.3      5.6      54                                     
67   Petrović             GKP  Bournemouth      4.5      5.5      2.6      52                                     
470  Dúbravka             GKP  Burnley          4.0      33.7     2.0      43                                     
736  Donnarumma           GKP  Man City         5.7      10.5     3.6      43                                     
139  Verbruggen           GKP  Brighton         4.4      5.0      4.2      41                                     
101  Kelleher             GKP  Brentford        4.5      7.6      2.0      40                                     
314  Leno                 GKP  Fulham           4.9      1.5      1.6      40                                     
502  Sels                 GKP  Nott'm Forest    4.7      7.7      3.8      40                                     
366  A.Becker             GKP  Liverpool        5.4      5.5      2.4      32                                     
600  Areola               GKP  West Ham         4.3      2.1      3.2      29                                     
665  Perri                GKP  Leeds            4.5      0.2      2.0      25                                     
733  Lammens              GKP  Man Utd          5.1      1.9      1.4      22                                     
627  Johnstone            GKP  Wolves           4.5      0.1      1.6      21

> fplr live 15 --limit 10
ID   Name                 TOTAL_POINTS
242  Dewsbury-Hall        16          
408  Rúben                14          
717  Xavi                 14          
661  Ekitiké              13          
403  Gvardiol             12          
414  Foden                12          
417  Cherki               12          
660  Stach                12          
226  Chalobah             11          
220  Sánchez              10

> fplr fixture
ID   Kickoff Time         Home                 Away                
154  2025-12-13T15:00:00Z Everton              Chelsea             
156  2025-12-13T15:00:00Z Brighton             Liverpool           
153  2025-12-13T17:30:00Z Fulham               Burnley             
151  2025-12-13T20:00:00Z Wolves               Arsenal             
155  2025-12-14T14:00:00Z Man City             Crystal Palace      
158  2025-12-14T14:00:00Z Spurs                Nott'm Forest       
159  2025-12-14T14:00:00Z Newcastle            Sunderland          
160  2025-12-14T14:00:00Z Aston Villa          West Ham            
152  2025-12-14T16:30:00Z Leeds                Brentford           
157  2025-12-15T20:00:00Z Bournemouth          Man Utd

> fplr gameweek
ID   Name             Status       Deadline            
1    Gameweek 1       Finished     2025-08-15T17:30:00Z
2    Gameweek 2       Finished     2025-08-22T17:30:00Z
3    Gameweek 3       Finished     2025-08-30T10:00:00Z
4    Gameweek 4       Finished     2025-09-13T10:00:00Z
5    Gameweek 5       Finished     2025-09-20T10:00:00Z
6    Gameweek 6       Finished     2025-09-27T10:00:00Z
7    Gameweek 7       Finished     2025-10-03T17:30:00Z
8    Gameweek 8       Finished     2025-10-18T10:00:00Z
9    Gameweek 9       Finished     2025-10-24T17:30:00Z
10   Gameweek 10      Finished     2025-11-01T13:30:00Z
11   Gameweek 11      Finished     2025-11-08T11:00:00Z
12   Gameweek 12      Finished     2025-11-22T11:00:00Z
13   Gameweek 13      Finished     2025-11-29T13:30:00Z
14   Gameweek 14      Finished     2025-12-02T18:00:00Z
15   Gameweek 15      Current      2025-12-06T11:00:00Z
16   Gameweek 16      Next         2025-12-13T13:30:00Z
17   Gameweek 17      Upcoming     2025-12-20T11:00:00Z
18   Gameweek 18      Upcoming     2025-12-26T18:30:00Z
19   Gameweek 19      Upcoming     2025-12-30T18:00:00Z
20   Gameweek 20      Upcoming     2026-01-03T11:00:00Z
21   Gameweek 21      Upcoming     2026-01-06T18:30:00Z
22   Gameweek 22      Upcoming     2026-01-17T11:00:00Z
23   Gameweek 23      Upcoming     2026-01-24T11:00:00Z
24   Gameweek 24      Upcoming     2026-01-31T13:30:00Z
25   Gameweek 25      Upcoming     2026-02-07T13:30:00Z
26   Gameweek 26      Upcoming     2026-02-11T18:30:00Z
27   Gameweek 27      Upcoming     2026-02-21T13:30:00Z
28   Gameweek 28      Upcoming     2026-02-28T13:30:00Z
29   Gameweek 29      Upcoming     2026-03-04T18:30:00Z
30   Gameweek 30      Upcoming     2026-03-14T13:30:00Z
31   Gameweek 31      Upcoming     2026-03-21T13:30:00Z
32   Gameweek 32      Upcoming     2026-04-11T12:30:00Z
33   Gameweek 33      Upcoming     2026-04-18T12:30:00Z
34   Gameweek 34      Upcoming     2026-04-25T12:30:00Z
35   Gameweek 35      Upcoming     2026-05-02T12:30:00Z
36   Gameweek 36      Upcoming     2026-05-09T12:30:00Z
37   Gameweek 37      Upcoming     2026-05-17T12:30:00Z
38   Gameweek 38      Upcoming     2026-05-24T13:30:00Z

> fplr team
ID   Name                 Short Name
1    Arsenal              ARS 
2    Aston Villa          AVL 
3    Burnley              BUR 
4    Bournemouth          BOU 
5    Brentford            BRE 
6    Brighton             BHA 
7    Chelsea              CHE 
8    Crystal Palace       CRY 
9    Everton              EVE 
10   Fulham               FUL 
11   Leeds                LEE 
12   Liverpool            LIV 
13   Man City             MCI 
14   Man Utd              MUN 
15   Newcastle            NEW 
16   Nott'm Forest        NFO 
17   Sunderland           SUN 
18   Spurs                TOT 
19   West Ham             WHU 
20   Wolves               WOL
```
