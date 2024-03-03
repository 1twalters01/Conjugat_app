for every verb_page_info_vec in verb_page_info_vec (so split into languages)
    for every verb_page_info in verb_page_info_vec (split by verb_page_info) - while loop instead as rust might not like the list increasing whilst running a for loop?
        put verb_page_info.metadata.similar_verbs and verb_page_info.metadata.other_verbs into a verb_vec
        for verb in verb_vec
            if verb in already_scraped_vec
                continue
            else
                create url
                scrape url
                add to the lists
