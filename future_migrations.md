1. **Add profile image column in profiles tables** ✅
2. **Convert thoughts from single TEXT block to Text Array** ✅
3. **Add a beta whitelisted users table** ✅
4. **Add a table for Admins** ✅
5. **Add a column on Profiles table to store the users password hash** ✅
6. **Add a column on Originals table to store the password_hash** ✅
7. **Create a watchlist table** ✅
8. **Add a column to the Originals table that contains the releases from the table** -- let's say if we consider every release as a work from teh original where artis_id andOriginal_ID is is the same then we can get the releases right and this releases will be considered as the works of the origianls right? -- this concludes we dont need another table or column about releases for the original right ah ✅ 
   -- Completed migration 2 --
9.  **Add metrics presence to the profiles table** - ✅
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
19. **Add separate tabel as for seasons** ❌- see we can make the season as a new original and add the parent as the same page created for the entire orignal, so take `The Boys` for example for series we create an original entry with the name `The Boys` and assoicated with prime_video somethig like that and then for every season we create an new original entry and we link the parent as the `the BOys` orignal for this entry so it points to the parent orignals and then for episodes this seanson-1-boys acts as the parent and we acheive it with the single table design and we also get the all the feed related to `boys` in the parent original and we can also get the feed related to seasona and even for episodes also - by this design parent column we can acheive the linking of the series to its seans and episodes in a single table of orignals but querying will be a bit complex and slow because for every episode lookup we have to do a complete orignals table lookup. -- verdict is we dont need separate table for seasons
20. **Add a OriginalType to watchlist** (SERIES,MOVIE,SEASON,EPISODE) - ✅
21. **Create a new column for type with check constraint on watchlist** - ✅
    --- completed migration 4 & 5 ---
22. **Rename Poster and Edit format TYPE in the DB to get rid of hyphens** - ✅
23. **ADD Work_views and work_likes tables** - ✅
    --- completed migration 6 --
24. **Add a column for storing the total views and likes on the Originals table** 
25. **Drop the created at column in the Works table** - since anyways we are storing the created_at on individual work tables we dont need to store it in the works table
26. 