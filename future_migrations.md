1. **Add profile image column in profiles tables** ✅
2. **Convert thoughts from single TEXT block to Text Array** ✅
3. **Add a beta whitelisted users table** ✅
4. **Add a table for Admins** ✅
5. **Add a column on Profiles table to store the users password hash** ✅
6. **Add a column on Originals table to store the password_hash** ✅
7. **Create a watchlist table** ✅
8. **Add a column to the Originals table that contains the releases from the table** -- let's say if we consider every release as a work from teh original where artis_id andOriginal_ID is is the same then we can get the releases right and this releases will be considered as the works of the origianls right? -- this concludes we dont need another table or column about releases for the original right ah ✅
   -- Completed migration 2 --
9. **Add metrics presence to the profiles table** - ✅
10. **Add NOT NULL constraint to the is_claimed column in profiles table** - ✅
11. **Add unique username constraint to the profiles table** - ✅
12. **Add a column on the profiles table to store user's status (STAR,MAKER,ARTIST)**` - ✅
13. **Remove Releases column from Originals table** -- from the `12` th point if we are adding the user Status/Type column to the profile's table then we can move the originals table to the profiles table anyways Original's table is just a subset of profiles table right? so whats the difference that this make like what is the point of having a separate table for originals if we are adding the user status column to the profiles table and we can easily filter out the original users from the profiles table itself right? -- this concludes that we can remove the Originals table and move all the columns to the profiles table and we can easily filter out the original users from the profiles table itself right? no no we cant go in this route because of queries will take more time to execute and for Originals we are going to add a feature that makers can be directly associated with the original and see it may be an simple feature but come to thnik of it for indie cinema and short films the original associated Production handle the Maker Stage will be presented differently than others. - ✅
14. **Add Associated with On Originals table** - ✅
    -- completed completed migration 3--
15. **Add Release Date and genre columns to the Originals table**
16. **Add Episodes Table**
17. **Add type on Orignals to differentiate movies and series**
18. **Add Parent Column on Originals table** - see this will be helpful to link the multiple movies in to collection like bahubali duology and it can also be used as a reference to the seasons and episodes in case of series right? and in the feed of this parent orignal we will have broader options to fill the feed and yeah - there is a work-around so we will maked the associated_with as the parent for children originals and for parents the associated_with is on of the maker right, so we can easily link but it i think its not a good design. - ✅
19. **Add separate tabel as for seasons** ❌- see we can make the season as a new original and add the parent as the same page created for the entire orignal, so take `The Boys` for example for series we create an original entry with the name `The Boys` and assoicated with prime_video somethig like that and then for every season we create an new original entry and we link the parent as the `the Boys` orignal for this entry so it points to the parent orignals and then for episodes this seanson-1-boys acts as the parent and we acheive it with the single table design and we also get the all the feed related to `boys` in the parent original and we can also get the feed related to seasona and even for episodes also - by this design parent column we can acheive the linking of the series to its seans and episodes in a single table of orignals but querying will be a bit complex and slow because for every episode lookup we have to do a complete orignals table lookup. -- verdict is we dont need separate table for seasons
20. **Add a OriginalType to watchlist** (SERIES,MOVIE,SEASON,EPISODE) - ✅
21. **Create a new column for type with check constraint on watchlist** - ✅
    --- completed migration 4 & 5 ---
22. **Rename Poster and Edit format TYPE in the DB to get rid of hyphens** - ✅
23. **ADD Work_views and work_likes tables** - ✅
    --- completed migration 6 --
24. **Add a Stage name column to the Profiles table** - ✅
25. **Add Text_color and Background_color columns to the profiles table** - ✅
26. **Drop the created at column in the Edits & Posters & Scripts Tables** - since anyways we are storing the created_at on individual work tables we dont need to store it in the works table - ✅
27. **Create a table for favorite profiles** - ✅
28. **Create a table for follows** - ✅
29. **Create table sets(title,statement,presence,description,curator)** - ✅
30. **Create table for festivals(title,start_date,end_date,Description,Essentials,Organizer)** - ✅
31. **Create table for panelists(festival_id,profile_id,work_id)** - ✅
32. **Create table for members(set_id,profile_id,Role)** - create a SetRole enum with values:member,moderator - ✅
33. **Create a table for festival works(festival_id,work_id)** - ✅
34. **Create a table for set_works(set_id,work_id)** - ✅
    --- completed migration 7 ---
35. **Add Profile picture column to the sets table** - ✅
36. **Add Unique Constraint to the sets name column** - ✅
37. **Add rules coloumn to festivals its a text** - ✅
38. **Add NOT NULL contraint to the pub_visibility column in the ledger table** - ✅
39. **Add NOT NULL constraint to the entry type column in the ledger table** - ✅
40. **Update the foreign key constraint on the original id to ON DELETE set null on ledger** - ✅
41. **Add created_at and updated_at columns to the ledger table** - ✅
42. **Remove the NOT NULL contraint on the work_id of the panelists table** - ✅
    --- completed migration 8 ---
43. **Remove the Not Null constraint on the original_id column in the originals_credits table and add a Reference column which points to profiles table and add a check to ensure that atleast one of them is present** - ✅
44. **ADD not null contraint to the created_at,updated_at,status columns int the ledger table** - ✅
45. **Add Not NULL contraint to the presence in the originals table** - ✅
46. **Add NOT NULL contraint to the profile picture column in sets table** - ✅
47. **Add NOT NULL contraint to the credits column in works table** - ✅
48. **Add NOT NULL constraint to the stage_name,text_color,background_color columns in profiles table** - ✅
49. **Rename roles table to cast_and_crew_roles** - ✅
50. **Add Unique constraint on the admin_name table okay** - ✅
    --- completed migration 9 ---
51. **Rename original_credits table to work_credits table**
52. **Drop the profile picture row from the sets table and add the text_color**
53. **Add film certification on the originals table**
54. **Create Recommendations table (Original_id,Artist_id,notes,created_at,updated_at,surge_score)**
55. **Update the artists table to add a column for current peak in recommednations & ledger**
56. **Add Resonance Density and Surge spread**
57. **Add indexes** - {Profile - user_name},{works-artist_id,works - created_at + id},{Edits,Scripts,Posters - work_id}