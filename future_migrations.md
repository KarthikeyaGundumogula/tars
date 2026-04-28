1. **Add profile image column in profiles tables** ✅
2. **Convert thoughts from single TEXT block to Text Array** ✅
3. **Add a beta whitelisted users table** ✅
4. **Add a table for Admins** ✅
5. **Add a column on Profiles table to store the users password hash** ✅
6. **Add a column on Originals table to store the password_hash** ✅
7. **Create a watchlist table** ✅
8. **Add a column to the Originals table that contains the releases from the table** -- let's say if we consider every release as a work from teh original where artis_id andOriginal_ID is is the same then we can get the releases right and this releases will be considered as the works of the origianls right? -- this concludes we dont need another table or column about releases for the original right ah ✅ 

   -- Completed --
9.  **Add metrics presence to the profiles table** - ✅
10. **Add NOT NULL constraint to the is_claimed column in profiles table** - ✅
11. **Add unique username constraint to the profiles table** - ✅
12. **Add a column on the profiles table to store user's status (STAR,MAKER,ARTIST)**` - ✅
13. **Remove Releases column from Originals table** -- from the `12` th point if we are adding the user Status/Type column to the profile's table then we can move the originals table to the profiles table anyways Original's table is just a subset of profiles table right? so whats the difference that this make like what is the point of having a separate table for originals if we are adding the user status column to the profiles table and we can easily filter out the original users from the profiles table itself right? -- this concludes that we can remove the Originals table and move all the columns to the profiles table and we can easily filter out the original users from the profiles table itself right? no no we cant go in this route because of queries will take more time to execute and for Originals we are going to add a feature that makers can be directly associated with the original and see it may be an simple feature but come to thnik of it for indie cinema and short films the original associated Production handle the Maker Stage will be presented differently than others. - ✅
14. **Add Associated with On Originals table** - ✅