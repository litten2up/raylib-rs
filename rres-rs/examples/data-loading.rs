use raylib::audio::RaylibAudio;

fn main()
{
    // Initialization
    //--------------------------------------------------------------------------------------
    const screenWidth: i32 = 384;
    const screenHeight:i32 = 512;

    let (mut rl, thread) = raylib::init().size(screenWidth, screenHeight).title("rres example - rres data loading").build();


    let audio = RaylibAudio::init_audio_device()?;

    rl.set_target_fps(60);               // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    while (!rl.window_should_close())    // Detect window close button or ESC key
    {
        // Dropped files logic
        //----------------------------------------------------------------------------------
        if (rl.is_file_dropped())
        {
            FilePathList droppedFiles = LoadDroppedFiles();

            if (IsFileExtension(droppedFiles.paths[0], ".rres"))
            {
                int result = 0;     // Result of data unpacking

                // TEST 01: Load rres Central Directory (RRES_DATA_DIRECTORY)
                //------------------------------------------------------------------------------------------------------
                rresCentralDir dir = rresLoadCentralDirectory(droppedFiles.paths[0]);

                // NOTE: By default central directory is never compressed/encrypted

                // Check if central directory is available
                // NOTE: CDIR is not mandatory, resources are referenced by its id
                if (dir.count == 0) TraceLog(LOG_WARNING, "No central directory available in the file");
                else
                {
                    // List all files contained on central directory
                    for (unsigned int i = 0; i < dir.count; i++)
                    {
                        TraceLog(LOG_INFO, "RRES: CDIR: File entry %03i: %s | Resource(s) id: 0x%08x | Offset: 0x%08x", i + 1, dir.entries[i].fileName, dir.entries[i].id, dir.entries[i].offset);
                    
                        // TODO: List contained resource chunks info
                        //rresResourceChunkInfo info = rresGetResourceChunkInfo(droppedFiles.paths[0], dir.entries[i]);
                    }
                }
                //------------------------------------------------------------------------------------------------------
                /*
                // TEST 02: Loading raw data (RRES_DATA_RAW)
                //------------------------------------------------------------------------------------------------------
                chunk = rresLoadResourceChunk(droppedFiles.paths[0], rresGetResourceId(dir, "resources/image.png.raw"));
                result = UnpackResourceChunk(&chunk);               // Decompres/decipher resource data (if required)

                if (result == 0)    // Data decompressed/decrypted successfully
                {
                    unsigned int dataSize = 0;
                    data = LoadDataFromResource(chunk, &dataSize);  // Load raw data, must be freed at the end

                    if ((data != NULL) && (dataSize > 0))
                    {
                        FILE *rawFile = fopen("export_data.raw", "wb");
                        fwrite(data, 1, dataSize, rawFile);
                        fclose(rawFile);
                    }
                }

                rresUnloadResourceChunk(chunk);
                //------------------------------------------------------------------------------------------------------

                // TEST 03: Load text data (RRES_DATA_TEXT)
                //------------------------------------------------------------------------------------------------------
                chunk = rresLoadResourceChunk(droppedFiles.paths[0], rresGetResourceId(dir, "resources/text_data.txt"));
                result = UnpackResourceChunk(&chunk);       // Decompres/decipher resource data (if required)

                if (result == 0)    // Data decompressed/decrypted successfully
                {
                    text = LoadTextFromResource(chunk);     // Load text data, must be freed at the end
                }

                rresUnloadResourceChunk(chunk);
                //------------------------------------------------------------------------------------------------------
                */
                
                // TEST 04: Load image data (RRES_DATA_IMAGE)
                //------------------------------------------------------------------------------------------------------
                chunk = rresLoadResourceChunk(droppedFiles.paths[0], rresGetResourceId(dir, "fudesumi.png"));
                result = UnpackResourceChunk(&chunk);       // Decompres/decipher resource data (if required)

                if (result == 0)    // Data decompressed/decrypted successfully
                {
                    Image image = LoadImageFromResource(chunk);
                    if (image.data != NULL)
                    {
                        texture = LoadTextureFromImage(image);
                        UnloadImage(image);
                    }
                }

                rresUnloadResourceChunk(chunk);
                //------------------------------------------------------------------------------------------------------

                // TEST 05: Load wave data (RRES_DATA_WAVE)
                //------------------------------------------------------------------------------------------------------
                chunk = rresLoadResourceChunk(droppedFiles.paths[0], rresGetResourceId(dir, "tanatana.ogg"));
                result = UnpackResourceChunk(&chunk);       // Decompres/decipher resource data (if required)

                if (result == 0)    // Data decompressed/decrypted successfully
                {
                    Wave wave = LoadWaveFromResource(chunk);
                    sound = LoadSoundFromWave(wave);
                    UnloadWave(wave);
                }

                rresUnloadResourceChunk(chunk);
                //------------------------------------------------------------------------------------------------------
                
                // TEST 06: Load font data, multiples chunks (RRES_DATA_FONT_GLYPHS + RRE_DATA_IMAGE)
                //------------------------------------------------------------------------------------------------------
                multi = rresLoadResourceMulti(droppedFiles.paths[0], rresGetResourceId(dir, "pixantiqua.ttf"));
                for (unsigned int i = 0; i < multi.count; i++)
                {
                    result = UnpackResourceChunk(&multi.chunks[i]);   // Decompres/decipher resource data (if required)
                    if (result != 0) break;
                }

                if (result == 0)    // All resources data decompressed/decrypted successfully
                {
                    font = LoadFontFromResource(multi);
                }
                
                rresUnloadResourceMulti(multi);
                //------------------------------------------------------------------------------------------------------
                
                // Unload central directory info, not required any more
                rresUnloadCentralDirectory(dir);
            }

            UnloadDroppedFiles(droppedFiles);    // Unload filepaths from memory
        }
        //----------------------------------------------------------------------------------

        // Update
        //----------------------------------------------------------------------------------
        // Play audio loaded from wave from .rres: RRES_DATA_WAVE
        if (IsKeyPressed(KEY_SPACE)) PlaySound(sound);
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        BeginDrawing();

            ClearBackground(RAYWHITE);

            DrawText("rres file loading: drag & drop a .rres file", 10, 10, 10, DARKGRAY);

            // Draw texture loaded from image from .rres: RRES_DATA_IMAGE
            DrawTexture(texture, 0, 0, WHITE);

            // Draw text using font loaded from .rres: RRES_DATA_FONT_GLYPHS + RRES_DATA_IMAGE
            DrawTextEx(font, "THIS IS a TEST!", (Vector2) { 10, 50 }, (float)font.baseSize, 0, RED);

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    MemFree(data);              // Unload raw data, using raylib memory allocator (same used by rres-raylib.h)
    MemFree(text);              // Unload text data, using raylib memory allocator (same used by rres-raylib.h)
    UnloadTexture(texture);     // Unload texture (VRAM)
    UnloadSound(sound);         // Unload sound
    UnloadFont(font);           // Unload font

    CloseAudioDevice();         // Close audio device

    CloseWindow();              // Close window and OpenGL context
    //--------------------------------------------------------------------------------------

    return 0;
}