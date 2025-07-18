static reg_errcode_t
byte_regex_compile (pattern, size, syntax, bufp)
     const char *pattern;
     size_t size;
     reg_syntax_t syntax;
     struct re_pattern_buffer *bufp;
{
  switch (x) {
        case '*':
          {
            boolean keep_string_p = 0;
            char zero_times_ok = 0, many_times_ok = 0;
            for (;;)
              {
                zero_times_ok |= c != '+';
                many_times_ok |= c != '?';
                if (p == pend)
                  break;
                do {if (p == pend) return REG_EEND; c = (unsigned char) *p++; if (translate) c = (unsigned char) translate[c]; } while (0);
                if (c == '*'
                    || (!(syntax & (((unsigned long int) 1) << 1)) && (c == '+' || c == '?')))
                  ;
                else if (c == '\'')
                  {
                  }
                else
                  {
                    p--;
                    break;
                  }
               }
            if (!laststart)
              break;
            if (many_times_ok)
              {
                ;
                while ((unsigned long) (b - bufp->buffer + (1 + 2)) > bufp->allocated) do { unsigned char *old_buffer = bufp->buffer; if (bufp->allocated == (1 << 16)) return REG_ESIZE; bufp->allocated <<= 1; if (bufp->allocated > (1 << 16)) bufp->allocated = (1 << 16); bufp->buffer = (unsigned char *) realloc ((bufp->buffer), (bufp->allocated)); if (bufp->buffer == ((void *)0)) return REG_ESPACE; if (old_buffer != bufp->buffer) { int incr = bufp->buffer - old_buffer; (b) += incr; (begalt) += incr; if (fixup_alt_jump) (fixup_alt_jump) += incr; if (laststart) (laststart) += incr; if (pending_exact) (pending_exact) += incr; } } while (0);
                if ((translate ? (char) translate[(unsigned char) (*(p - 2))] : (*(p - 2))) == (translate ? (char) translate[(unsigned char) ('.')] : ('.'))
      && zero_times_ok
                    && p < pend && (translate ? (char) translate[(unsigned char) (*p)] : (*p)) == (translate ? (char) translate[(unsigned char) ('\n')] : ('\n'))
                    && !(syntax & ((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1)))
                  {
                    byte_store_op1 (jump, b, (int) ((laststart) - (b) - (1 + 2)));
                    keep_string_p = 1;
                  }
                else
                  byte_store_op1 (maybe_pop_jump, b, (int) ((laststart - (1 + 2)) - (b) - (1 + 2)))
                                   ;
                b += 1 + 2;
              }
            while ((unsigned long) (b - bufp->buffer + (1 + 2)) > bufp->allocated) do { unsigned char *old_buffer = bufp->buffer; if (bufp->allocated == (1 << 16)) return REG_ESIZE; bufp->allocated <<= 1; if (bufp->allocated > (1 << 16)) bufp->allocated = (1 << 16); bufp->buffer = (unsigned char *) realloc ((bufp->buffer), (bufp->allocated)); if (bufp->buffer == ((void *)0)) return REG_ESPACE; if (old_buffer != bufp->buffer) { int incr = bufp->buffer - old_buffer; (b) += incr; (begalt) += incr; if (fixup_alt_jump) (fixup_alt_jump) += incr; if (laststart) (laststart) += incr; if (pending_exact) (pending_exact) += incr; } } while (0);
            byte_insert_op1 (keep_string_p ? on_failure_keep_string_jump : on_failure_jump, laststart, (int) ((b + 1 + 2) - (laststart) - (1 + 2)), b)
                                                                ;
            pending_exact = 0;
            b += 1 + 2;
            if (!zero_times_ok)
              {
                while ((unsigned long) (b - bufp->buffer + (1 + 2)) > bufp->allocated) do { unsigned char *old_buffer = bufp->buffer; if (bufp->allocated == (1 << 16)) return REG_ESIZE; bufp->allocated <<= 1; if (bufp->allocated > (1 << 16)) bufp->allocated = (1 << 16); bufp->buffer = (unsigned char *) realloc ((bufp->buffer), (bufp->allocated)); if (bufp->buffer == ((void *)0)) return REG_ESPACE; if (old_buffer != bufp->buffer) { int incr = bufp->buffer - old_buffer; (b) += incr; (begalt) += incr; if (fixup_alt_jump) (fixup_alt_jump) += incr; if (laststart) (laststart) += incr; if (pending_exact) (pending_exact) += incr; } } while (0);
                byte_insert_op1 (dummy_failure_jump, laststart, (int) ((laststart + 2 + 2 * 2) - (laststart) - (1 + 2)), b)
                                    ;
                b += 1 + 2;
              }
            }
   break;
 case '.':
          laststart = b;
          do { while ((unsigned long) (b - bufp->buffer + (1)) > bufp->allocated) do { unsigned char *old_buffer = bufp->buffer; if (bufp->allocated == (1 << 16)) return REG_ESIZE; bufp->allocated <<= 1; if (bufp->allocated > (1 << 16)) bufp->allocated = (1 << 16); bufp->buffer = (unsigned char *) realloc ((bufp->buffer), (bufp->allocated)); if (bufp->buffer == ((void *)0)) return REG_ESPACE; if (old_buffer != bufp->buffer) { int incr = bufp->buffer - old_buffer; (b) += incr; (begalt) += incr; if (fixup_alt_jump) (fixup_alt_jump) += incr; if (laststart) (laststart) += incr; if (pending_exact) (pending_exact) += incr; } } while (0); *b++ = (unsigned char) (anychar); } while (0);
          break;
        case '[':
          {
            boolean had_char_class = 0;
     unsigned int range_start = 0xffffffff;
            if (p == pend) return (free (compile_stack.stack), REG_EBRACK);
     while ((unsigned long) (b - bufp->buffer + (34)) > bufp->allocated) do { unsigned char *old_buffer = bufp->buffer; if (bufp->allocated == (1 << 16)) return REG_ESIZE; bufp->allocated <<= 1; if (bufp->allocated > (1 << 16)) bufp->allocated = (1 << 16); bufp->buffer = (unsigned char *) realloc ((bufp->buffer), (bufp->allocated)); if (bufp->buffer == ((void *)0)) return REG_ESPACE; if (old_buffer != bufp->buffer) { int incr = bufp->buffer - old_buffer; (b) += incr; (begalt) += incr; if (fixup_alt_jump) (fixup_alt_jump) += incr; if (laststart) (laststart) += incr; if (pending_exact) (pending_exact) += incr; } } while (0);
            laststart = b;
            do { while ((unsigned long) (b - bufp->buffer + (1)) > bufp->allocated) do { unsigned char *old_buffer = bufp->buffer; if (bufp->allocated == (1 << 16)) return REG_ESIZE; bufp->allocated <<= 1; if (bufp->allocated > (1 << 16)) bufp->allocated = (1 << 16); bufp->buffer = (unsigned char *) realloc ((bufp->buffer), (bufp->allocated)); if (bufp->buffer == ((void *)0)) return REG_ESPACE; if (old_buffer != bufp->buffer) { int incr = bufp->buffer - old_buffer; (b) += incr; (begalt) += incr; if (fixup_alt_jump) (fixup_alt_jump) += incr; if (laststart) (laststart) += incr; if (pending_exact) (pending_exact) += incr; } } while (0); *b++ = (unsigned char) (*p == '^' ? charset_not : charset); } while (0);
            if (*p == '^')
              p++;
            p1 = p;
            do { while ((unsigned long) (b - bufp->buffer + (1)) > bufp->allocated) do { unsigned char *old_buffer = bufp->buffer; if (bufp->allocated == (1 << 16)) return REG_ESIZE; bufp->allocated <<= 1; if (bufp->allocated > (1 << 16)) bufp->allocated = (1 << 16); bufp->buffer = (unsigned char *) realloc ((bufp->buffer), (bufp->allocated)); if (bufp->buffer == ((void *)0)) return REG_ESPACE; if (old_buffer != bufp->buffer) { int incr = bufp->buffer - old_buffer; (b) += incr; (begalt) += incr; if (fixup_alt_jump) (fixup_alt_jump) += incr; if (laststart) (laststart) += incr; if (pending_exact) (pending_exact) += incr; } } while (0); *b++ = (unsigned char) ((1 << 8) / 8); } while (0);
            (memset (b, 0, (1 << 8) / 8), (b));
            if ((re_opcode_t) b[-2] == charset_not
                && (syntax & ((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1)))
              (b[((unsigned char) ('\n')) / 8] |= 1 << (((unsigned char) '\n') % 8));
            for (;;)
              {
                if (p == pend) return (free (compile_stack.stack), REG_EBRACK);
                do {if (p == pend) return REG_EEND; c = (unsigned char) *p++; if (translate) c = (unsigned char) translate[c]; } while (0);
                if ((syntax & ((unsigned long int) 1)) && c == 'a')
                  {
                    if (p == pend) return (free (compile_stack.stack), REG_EESCAPE);
                    do {if (p == pend) return REG_EEND; c1 = (unsigned char) *p++; if (translate) c1 = (unsigned char) translate[c1]; } while (0);
                    (b[((unsigned char) (c1)) / 8] |= 1 << (((unsigned char) c1) % 8));
      range_start = c1;
                    continue;
                  }
                if (c == ']' && p != p1 + 1)
                  break;
                if (had_char_class && c == '-' && *p != ']')
                  return (free (compile_stack.stack), REG_ERANGE);
                if (c == '-'
                    && !(p - 2 >= pattern && p[-2] == '[')
                    && !(p - 3 >= pattern && p[-3] == '[' && p[-2] == '^')
                    && *p != ']')
                  {
                    reg_errcode_t ret
                      = byte_compile_range (range_start, &p, pend, translate,
         syntax, b);
                    if (ret != REG_NOERROR) return (free (compile_stack.stack), ret);
      range_start = 0xffffffff;
                  }
                else if (p[0] == '-' && p[1] != ']')
                  {
                    reg_errcode_t ret;
                    do {if (p == pend) return REG_EEND; c1 = (unsigned char) *p++; if (translate) c1 = (unsigned char) translate[c1]; } while (0);
                    ret = byte_compile_range (c, &p, pend, translate, syntax, b);
                    if (ret != REG_NOERROR) return (free (compile_stack.stack), ret);
      range_start = 0xffffffff;
                  }
                else if (syntax & ((((unsigned long int) 1) << 1) << 1) && c == '[' && *p == ':')
                  {
                    char str[6 + 1];
                    do {if (p == pend) return REG_EEND; c = (unsigned char) *p++; if (translate) c = (unsigned char) translate[c]; } while (0);
                    c1 = 0;
                    if (p == pend) return (free (compile_stack.stack), REG_EBRACK);
                    for (;;)
                      {
                        do {if (p == pend) return REG_EEND; c = (unsigned char) *p++; if (translate) c = (unsigned char) translate[c]; } while (0);
                        if ((c == ':' && *p == ']') || p == pend)
                          break;
   if (c1 < 6)
     str[c1++] = c;
   else
     str[0] = 0;
                      }
                    str[c1] = 0;
                    if (c == ':' && *p == ']')
                      {
                        int ch;
                        boolean is_alnum = ((strcmp (str, "alnum") == 0));
                        boolean is_alpha = ((strcmp (str, "alpha") == 0));
                        boolean is_blank = ((strcmp (str, "blank") == 0));
                        boolean is_cntrl = ((strcmp (str, "cntrl") == 0));
                        boolean is_digit = ((strcmp (str, "digit") == 0));
                        boolean is_graph = ((strcmp (str, "graph") == 0));
                        boolean is_lower = ((strcmp (str, "lower") == 0));
                        boolean is_print = ((strcmp (str, "print") == 0));
                        boolean is_punct = ((strcmp (str, "punct") == 0));
                        boolean is_space = ((strcmp (str, "space") == 0));
                        boolean is_upper = ((strcmp (str, "upper") == 0));
                        boolean is_xdigit = ((strcmp (str, "xdigit") == 0));
                        if (!(((strcmp (str, "alpha") == 0)) || ((strcmp (str, "upper") == 0)) || ((strcmp (str, "lower") == 0)) || ((strcmp (str, "digit") == 0)) || ((strcmp (str, "alnum") == 0)) || ((strcmp (str, "xdigit") == 0)) || ((strcmp (str, "space") == 0)) || ((strcmp (str, "print") == 0)) || ((strcmp (str, "punct") == 0)) || ((strcmp (str, "graph") == 0)) || ((strcmp (str, "cntrl") == 0)) || ((strcmp (str, "blank") == 0))))
     return (free (compile_stack.stack), REG_ECTYPE);
                        do {if (p == pend) return REG_EEND; c = (unsigned char) *p++; if (translate) c = (unsigned char) translate[c]; } while (0);
                        if (p == pend) return (free (compile_stack.stack), REG_EBRACK);
                        for (ch = 0; ch < 1 << 8; ch++)
                          {
                            if ( (is_alnum && (1 && ((*__ctype_b_loc ())[(int) ((ch))] & (unsigned short int) _ISalnum)))
                                || (is_alpha && (1 && ((*__ctype_b_loc ())[(int) ((ch))] & (unsigned short int) _ISalpha)))
                                || (is_blank && (1 && ((*__ctype_b_loc ())[(int) ((ch))] & (unsigned short int) _ISblank)))
                                || (is_cntrl && (1 && ((*__ctype_b_loc ())[(int) ((ch))] & (unsigned short int) _IScntrl))))
         (b[((unsigned char) (ch)) / 8] |= 1 << (((unsigned char) ch) % 8));
       if ( (is_digit && (1 && ((*__ctype_b_loc ())[(int) ((ch))] & (unsigned short int) _ISdigit)))
                                || (is_graph && (1 && ((*__ctype_b_loc ())[(int) ((ch))] & (unsigned short int) _ISgraph)))
                                || (is_lower && (1 && ((*__ctype_b_loc ())[(int) ((ch))] & (unsigned short int) _ISlower)))
                                || (is_print && (1 && ((*__ctype_b_loc ())[(int) ((ch))] & (unsigned short int) _ISprint))))
         (b[((unsigned char) (ch)) / 8] |= 1 << (((unsigned char) ch) % 8));
       if ( (is_punct && (1 && ((*__ctype_b_loc ())[(int) ((ch))] & (unsigned short int) _ISpunct)))
                                || (is_space && (1 && ((*__ctype_b_loc ())[(int) ((ch))] & (unsigned short int) _ISspace)))
                                || (is_upper && (1 && ((*__ctype_b_loc ())[(int) ((ch))] & (unsigned short int) _ISupper)))
                                || (is_xdigit && (1 && ((*__ctype_b_loc ())[(int) ((ch))] & (unsigned short int) _ISxdigit))))
         (b[((unsigned char) (ch)) / 8] |= 1 << (((unsigned char) ch) % 8));
       if ( translate && (is_upper || is_lower)
    && ((1 && ((*__ctype_b_loc ())[(int) ((ch))] & (unsigned short int) _ISupper)) || (1 && ((*__ctype_b_loc ())[(int) ((ch))] & (unsigned short int) _ISlower))))
         (b[((unsigned char) (ch)) / 8] |= 1 << (((unsigned char) ch) % 8));
                          }
                        had_char_class = 1;
                      }
                    else
                      {
                        c1++;
                        while (c1--)
                          p--;
                        (b[((unsigned char) ('[')) / 8] |= 1 << (((unsigned char) '[') % 8));
                        (b[((unsigned char) (':')) / 8] |= 1 << (((unsigned char) ':') % 8));
   range_start = ':';
                        had_char_class = 0;
                      }
                  }
                else if (syntax & ((((unsigned long int) 1) << 1) << 1) && c == '[' && *p == '=')
    {
      unsigned char str[16 + 1];
      do {if (p == pend) return REG_EEND; c = (unsigned char) *p++; if (translate) c = (unsigned char) translate[c]; } while (0);
      c1 = 0;
      if (p == pend) return (free (compile_stack.stack), REG_EBRACK);
      for (;;)
        {
   do {if (p == pend) return REG_EEND; c = (unsigned char) *p++; if (translate) c = (unsigned char) translate[c]; } while (0);
   if ((c == '=' && *p == ']') || p == pend)
     break;
   if (c1 < 16)
     str[c1++] = c;
   else
     str[0] = 0;
                      }
      str[c1] = 0;
      if (c == '=' && *p == ']' && str[0] != 0)
        {
     {
       if (c1 != 1)
         return (free (compile_stack.stack), REG_ECOLLATE);
       do {if (p == pend) return REG_EEND; c = (unsigned char) *p++; if (translate) c = (unsigned char) translate[c]; } while (0);
       (b[((unsigned char) (str[0])) / 8] |= 1 << (((unsigned char) str[0]) % 8));
     }
   had_char_class = 1;
        }
                    else
                      {
                        c1++;
                        while (c1--)
                          p--;
                        (b[((unsigned char) ('[')) / 8] |= 1 << (((unsigned char) '[') % 8));
                        (b[((unsigned char) ('=')) / 8] |= 1 << (((unsigned char) '=') % 8));
   range_start = '=';
                        had_char_class = 0;
                      }
    }
                else if (syntax & ((((unsigned long int) 1) << 1) << 1) && c == '[' && *p == '.')
    {
      unsigned char str[128];
      do {if (p == pend) return REG_EEND; c = (unsigned char) *p++; if (translate) c = (unsigned char) translate[c]; } while (0);
      c1 = 0;
      if (p == pend) return (free (compile_stack.stack), REG_EBRACK);
      for (;;)
        {
   do {if (p == pend) return REG_EEND; c = (unsigned char) *p++; if (translate) c = (unsigned char) translate[c]; } while (0);
   if ((c == '.' && *p == ']') || p == pend)
     break;
   if (c1 < sizeof (str))
     str[c1++] = c;
   else
     str[0] = 0;
                      }
      str[c1] = 0;
      if (c == '.' && *p == ']' && str[0] != 0)
        {
     {
       if (c1 != 1)
         return (free (compile_stack.stack), REG_ECOLLATE);
       do {if (p == pend) return REG_EEND; c = (unsigned char) *p++; if (translate) c = (unsigned char) translate[c]; } while (0);
       (b[((unsigned char) (str[0])) / 8] |= 1 << (((unsigned char) str[0]) % 8));
       range_start = ((const unsigned char *) str)[0];
     }
   had_char_class = 0;
        }
                    else
                      {
                        c1++;
                        while (c1--)
                          p--;
                        (b[((unsigned char) ('[')) / 8] |= 1 << (((unsigned char) '[') % 8));
                        (b[((unsigned char) ('.')) / 8] |= 1 << (((unsigned char) '.') % 8));
   range_start = '.';
                        had_char_class = 0;
                      }
    }
                else
                  {
                    had_char_class = 0;
                    (b[((unsigned char) (c)) / 8] |= 1 << (((unsigned char) c) % 8));
      range_start = c;
                  }
              }
            while ((int) b[-1] > 0 && b[b[-1] - 1] == 0)
              b[-1]--;
            b += b[-1];
          }
          break;
 case '(':
          if (syntax & (((((((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1))
            goto handle_open;
          else
            goto normal_char;
        case ')':
          if (syntax & (((((((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1))
            goto handle_close;
          else
            goto normal_char;
        case '\n':
          if (syntax & (((((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1))
            goto handle_alt;
          else
            goto normal_char;
 case '|':
          if (syntax & (((((((((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1))
            goto handle_alt;
          else
            goto normal_char;
        case '{':
           if (syntax & (((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) && syntax & ((((((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1))
             goto handle_interval;
           else
             goto normal_char;
        case 'a':
          if (p == pend) return (free (compile_stack.stack), REG_EESCAPE);
          do {if (p == pend) return REG_EEND; c = (unsigned char) *p++; } while (0);
          switch (c)
            {
            case '(':
              if (syntax & (((((((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1))
                goto normal_backslash;
            handle_open:
              bufp->re_nsub++;
              regnum++;
              if ((compile_stack.avail == compile_stack.size))
                {
                  ((compile_stack.stack) = (compile_stack_elt_t *) realloc (compile_stack.stack, (compile_stack.size << 1) * sizeof (compile_stack_elt_t)))
                                                ;
                  if (compile_stack.stack == ((void *)0)) return REG_ESPACE;
                  compile_stack.size <<= 1;
                }
              (compile_stack.stack[compile_stack.avail]).begalt_offset = begalt - bufp->buffer;
              (compile_stack.stack[compile_stack.avail]).fixup_alt_jump
                = fixup_alt_jump ? fixup_alt_jump - bufp->buffer + 1 : 0;
              (compile_stack.stack[compile_stack.avail]).laststart_offset = b - bufp->buffer;
              (compile_stack.stack[compile_stack.avail]).regnum = regnum;
              if (regnum <= 255)
                {
                  (compile_stack.stack[compile_stack.avail]).inner_group_offset = b
      - bufp->buffer + 2;
                  do { while ((unsigned long) (b - bufp->buffer + (3)) > bufp->allocated) do { unsigned char *old_buffer = bufp->buffer; if (bufp->allocated == (1 << 16)) return REG_ESIZE; bufp->allocated <<= 1; if (bufp->allocated > (1 << 16)) bufp->allocated = (1 << 16); bufp->buffer = (unsigned char *) realloc ((bufp->buffer), (bufp->allocated)); if (bufp->buffer == ((void *)0)) return REG_ESPACE; if (old_buffer != bufp->buffer) { int incr = bufp->buffer - old_buffer; (b) += incr; (begalt) += incr; if (fixup_alt_jump) (fixup_alt_jump) += incr; if (laststart) (laststart) += incr; if (pending_exact) (pending_exact) += incr; } } while (0); *b++ = (unsigned char) (start_memory); *b++ = (unsigned char) (regnum); *b++ = (unsigned char) (0); } while (0);
                }
              compile_stack.avail++;
              fixup_alt_jump = 0;
              laststart = 0;
              begalt = b;
       pending_exact = 0;
              break;
            case ')':
              if (syntax & (((((((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1)) goto normal_backslash;
              if ((compile_stack.avail == 0))
  {
    if (syntax & (((((((((((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1))
      goto normal_backslash;
    else
      return (free (compile_stack.stack), REG_ERPAREN);
  }
            handle_close:
              if (fixup_alt_jump)
                {
                  do { while ((unsigned long) (b - bufp->buffer + (1)) > bufp->allocated) do { unsigned char *old_buffer = bufp->buffer; if (bufp->allocated == (1 << 16)) return REG_ESIZE; bufp->allocated <<= 1; if (bufp->allocated > (1 << 16)) bufp->allocated = (1 << 16); bufp->buffer = (unsigned char *) realloc ((bufp->buffer), (bufp->allocated)); if (bufp->buffer == ((void *)0)) return REG_ESPACE; if (old_buffer != bufp->buffer) { int incr = bufp->buffer - old_buffer; (b) += incr; (begalt) += incr; if (fixup_alt_jump) (fixup_alt_jump) += incr; if (laststart) (laststart) += incr; if (pending_exact) (pending_exact) += incr; } } while (0); *b++ = (unsigned char) (push_dummy_failure); } while (0);
                  byte_store_op1 (jump_past_alt, fixup_alt_jump, (int) ((b - 1) - (fixup_alt_jump) - (1 + 2)));
                }
              if ((compile_stack.avail == 0))
  {
    if (syntax & (((((((((((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1))
      goto normal_char;
    else
      return (free (compile_stack.stack), REG_ERPAREN);
  }
              ;
              {
                regnum_t this_group_regnum;
                compile_stack.avail--;
                begalt = bufp->buffer + (compile_stack.stack[compile_stack.avail]).begalt_offset;
                fixup_alt_jump
                  = (compile_stack.stack[compile_stack.avail]).fixup_alt_jump
                    ? bufp->buffer + (compile_stack.stack[compile_stack.avail]).fixup_alt_jump - 1
                    : 0;
                laststart = bufp->buffer + (compile_stack.stack[compile_stack.avail]).laststart_offset;
                this_group_regnum = (compile_stack.stack[compile_stack.avail]).regnum;
  pending_exact = 0;
                if (this_group_regnum <= 255)
                  {
      unsigned char *inner_group_loc
                      = bufp->buffer + (compile_stack.stack[compile_stack.avail]).inner_group_offset;
                    *inner_group_loc = regnum - this_group_regnum;
                    do { while ((unsigned long) (b - bufp->buffer + (3)) > bufp->allocated) do { unsigned char *old_buffer = bufp->buffer; if (bufp->allocated == (1 << 16)) return REG_ESIZE; bufp->allocated <<= 1; if (bufp->allocated > (1 << 16)) bufp->allocated = (1 << 16); bufp->buffer = (unsigned char *) realloc ((bufp->buffer), (bufp->allocated)); if (bufp->buffer == ((void *)0)) return REG_ESPACE; if (old_buffer != bufp->buffer) { int incr = bufp->buffer - old_buffer; (b) += incr; (begalt) += incr; if (fixup_alt_jump) (fixup_alt_jump) += incr; if (laststart) (laststart) += incr; if (pending_exact) (pending_exact) += incr; } } while (0); *b++ = (unsigned char) (stop_memory); *b++ = (unsigned char) (this_group_regnum); *b++ = (unsigned char) (regnum - this_group_regnum); } while (0)
                                                           ;
                  }
              }
              break;
            case '|':
              if (syntax & ((((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) || syntax & (((((((((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1))
                goto normal_backslash;
            handle_alt:
              if (syntax & ((((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1))
                goto normal_char;
              while ((unsigned long) (b - bufp->buffer + (1 + 2)) > bufp->allocated) do { unsigned char *old_buffer = bufp->buffer; if (bufp->allocated == (1 << 16)) return REG_ESIZE; bufp->allocated <<= 1; if (bufp->allocated > (1 << 16)) bufp->allocated = (1 << 16); bufp->buffer = (unsigned char *) realloc ((bufp->buffer), (bufp->allocated)); if (bufp->buffer == ((void *)0)) return REG_ESPACE; if (old_buffer != bufp->buffer) { int incr = bufp->buffer - old_buffer; (b) += incr; (begalt) += incr; if (fixup_alt_jump) (fixup_alt_jump) += incr; if (laststart) (laststart) += incr; if (pending_exact) (pending_exact) += incr; } } while (0);
              byte_insert_op1 (on_failure_jump, begalt, (int) ((b + 2 + 2 * 2) - (begalt) - (1 + 2)), b)
                                      ;
              pending_exact = 0;
              b += 1 + 2;
              if (fixup_alt_jump)
                byte_store_op1 (jump_past_alt, fixup_alt_jump, (int) ((b) - (fixup_alt_jump) - (1 + 2)));
              fixup_alt_jump = b;
              while ((unsigned long) (b - bufp->buffer + (1 + 2)) > bufp->allocated) do { unsigned char *old_buffer = bufp->buffer; if (bufp->allocated == (1 << 16)) return REG_ESIZE; bufp->allocated <<= 1; if (bufp->allocated > (1 << 16)) bufp->allocated = (1 << 16); bufp->buffer = (unsigned char *) realloc ((bufp->buffer), (bufp->allocated)); if (bufp->buffer == ((void *)0)) return REG_ESPACE; if (old_buffer != bufp->buffer) { int incr = bufp->buffer - old_buffer; (b) += incr; (begalt) += incr; if (fixup_alt_jump) (fixup_alt_jump) += incr; if (laststart) (laststart) += incr; if (pending_exact) (pending_exact) += incr; } } while (0);
              b += 1 + 2;
              laststart = 0;
              begalt = b;
              break;
            case '{':
              if (!(syntax & (((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1))
    || (syntax & ((((((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1)))
                goto normal_backslash;
            handle_interval:
              {
                int lower_bound = -1, upper_bound = -1;
  const char *beg_interval = p;
                if (p == pend)
    goto invalid_interval;
                { while (p != pend) { do {if (p == pend) return REG_EEND; c = (unsigned char) *p++; if (translate) c = (unsigned char) translate[c]; } while (0); if (c < '0' || c > '9') break; if (lower_bound <= (0x7fff)) { if (lower_bound < 0) lower_bound = 0; lower_bound = lower_bound * 10 + c - '0'; } } };
                if (c == ',')
                  {
                    { while (p != pend) { do {if (p == pend) return REG_EEND; c = (unsigned char) *p++; if (translate) c = (unsigned char) translate[c]; } while (0); if (c < '0' || c > '9') break; if (upper_bound <= (0x7fff)) { if (upper_bound < 0) upper_bound = 0; upper_bound = upper_bound * 10 + c - '0'; } } };
      if (upper_bound < 0)
        upper_bound = (0x7fff);
                  }
                else
                  upper_bound = lower_bound;
                if (! (0 <= lower_bound && lower_bound <= upper_bound))
    goto invalid_interval;
                if (!(syntax & ((((((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1)))
                  {
      if (c != 'a' || p == pend)
        goto invalid_interval;
                    do {if (p == pend) return REG_EEND; c = (unsigned char) *p++; if (translate) c = (unsigned char) translate[c]; } while (0);
                  }
                if (c != '}')
    goto invalid_interval;
                if (!laststart)
                  {
      if (syntax & (((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1)
   && !(syntax & (((((((((((((((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1)))
                      return (free (compile_stack.stack), REG_BADRPT);
                    else if (syntax & ((((((unsigned long int) 1) << 1) << 1) << 1) << 1))
                      laststart = b;
                    else
                      goto unfetch_interval;
                  }
                if ((0x7fff) < upper_bound)
    return (free (compile_stack.stack), REG_BADBR);
                 if (upper_bound == 0)
                   {
                     while ((unsigned long) (b - bufp->buffer + (1 + 2)) > bufp->allocated) do { unsigned char *old_buffer = bufp->buffer; if (bufp->allocated == (1 << 16)) return REG_ESIZE; bufp->allocated <<= 1; if (bufp->allocated > (1 << 16)) bufp->allocated = (1 << 16); bufp->buffer = (unsigned char *) realloc ((bufp->buffer), (bufp->allocated)); if (bufp->buffer == ((void *)0)) return REG_ESPACE; if (old_buffer != bufp->buffer) { int incr = bufp->buffer - old_buffer; (b) += incr; (begalt) += incr; if (fixup_alt_jump) (fixup_alt_jump) += incr; if (laststart) (laststart) += incr; if (pending_exact) (pending_exact) += incr; } } while (0);
                     byte_insert_op1 (jump, laststart, (int) ((b + 1 + 2) - (laststart) - (1 + 2)), b)
                            ;
                     b += 1 + 2;
                   }
                 else
                   {
                     unsigned nbytes = 2 + 4 * 2 +
         (upper_bound > 1) * (2 + 4 * 2);
                     while ((unsigned long) (b - bufp->buffer + (nbytes)) > bufp->allocated) do { unsigned char *old_buffer = bufp->buffer; if (bufp->allocated == (1 << 16)) return REG_ESIZE; bufp->allocated <<= 1; if (bufp->allocated > (1 << 16)) bufp->allocated = (1 << 16); bufp->buffer = (unsigned char *) realloc ((bufp->buffer), (bufp->allocated)); if (bufp->buffer == ((void *)0)) return REG_ESPACE; if (old_buffer != bufp->buffer) { int incr = bufp->buffer - old_buffer; (b) += incr; (begalt) += incr; if (fixup_alt_jump) (fixup_alt_jump) += incr; if (laststart) (laststart) += incr; if (pending_exact) (pending_exact) += incr; } } while (0);
                     byte_insert_op2 (succeed_n, laststart, (int) ((b + 1 + 2 * 2 + (upper_bound > 1) * (1 + 2 * 2)) - (laststart) - (1 + 2)), lower_bound, b)
                     ;
                     b += 1 + 2 * 2;
                     byte_insert_op2 (set_number_at, laststart, 1
     + 2 * 2, lower_bound, b);
                     b += 1 + 2 * 2;
                     if (upper_bound > 1)
                       {
                         byte_store_op2 (jump_n, b, (int) ((laststart + 2 * 2 + 1) - (b) - (1 + 2)), upper_bound - 1)
                                                      ;
                         b += 1 + 2 * 2;
                         byte_insert_op2 (set_number_at, laststart,
          b - laststart,
          upper_bound - 1, b);
                         b += 1 + 2 * 2;
                       }
                   }
                pending_exact = 0;
  break;
       invalid_interval:
  if (!(syntax & (((((((((((((((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1)))
    return (free (compile_stack.stack), p == pend ? REG_EBRACE : REG_BADBR);
       unfetch_interval:
  p = beg_interval;
  c = '{';
  if (syntax & ((((((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1))
    goto normal_char;
  else
    goto normal_backslash;
       }
            case 'w':
       if (syntax & (((((((((((((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1))
  goto normal_char;
              laststart = b;
              do { while ((unsigned long) (b - bufp->buffer + (1)) > bufp->allocated) do { unsigned char *old_buffer = bufp->buffer; if (bufp->allocated == (1 << 16)) return REG_ESIZE; bufp->allocated <<= 1; if (bufp->allocated > (1 << 16)) bufp->allocated = (1 << 16); bufp->buffer = (unsigned char *) realloc ((bufp->buffer), (bufp->allocated)); if (bufp->buffer == ((void *)0)) return REG_ESPACE; if (old_buffer != bufp->buffer) { int incr = bufp->buffer - old_buffer; (b) += incr; (begalt) += incr; if (fixup_alt_jump) (fixup_alt_jump) += incr; if (laststart) (laststart) += incr; if (pending_exact) (pending_exact) += incr; } } while (0); *b++ = (unsigned char) (wordchar); } while (0);
              break;
            case 'W':
       if (syntax & (((((((((((((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1))
  goto normal_char;
              laststart = b;
              do { while ((unsigned long) (b - bufp->buffer + (1)) > bufp->allocated) do { unsigned char *old_buffer = bufp->buffer; if (bufp->allocated == (1 << 16)) return REG_ESIZE; bufp->allocated <<= 1; if (bufp->allocated > (1 << 16)) bufp->allocated = (1 << 16); bufp->buffer = (unsigned char *) realloc ((bufp->buffer), (bufp->allocated)); if (bufp->buffer == ((void *)0)) return REG_ESPACE; if (old_buffer != bufp->buffer) { int incr = bufp->buffer - old_buffer; (b) += incr; (begalt) += incr; if (fixup_alt_jump) (fixup_alt_jump) += incr; if (laststart) (laststart) += incr; if (pending_exact) (pending_exact) += incr; } } while (0); *b++ = (unsigned char) (notwordchar); } while (0);
              break;
            case '<':
       if (syntax & (((((((((((((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1))
  goto normal_char;
              do { while ((unsigned long) (b - bufp->buffer + (1)) > bufp->allocated) do { unsigned char *old_buffer = bufp->buffer; if (bufp->allocated == (1 << 16)) return REG_ESIZE; bufp->allocated <<= 1; if (bufp->allocated > (1 << 16)) bufp->allocated = (1 << 16); bufp->buffer = (unsigned char *) realloc ((bufp->buffer), (bufp->allocated)); if (bufp->buffer == ((void *)0)) return REG_ESPACE; if (old_buffer != bufp->buffer) { int incr = bufp->buffer - old_buffer; (b) += incr; (begalt) += incr; if (fixup_alt_jump) (fixup_alt_jump) += incr; if (laststart) (laststart) += incr; if (pending_exact) (pending_exact) += incr; } } while (0); *b++ = (unsigned char) (wordbeg); } while (0);
              break;
            case '>':
       if (syntax & (((((((((((((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1))
  goto normal_char;
              do { while ((unsigned long) (b - bufp->buffer + (1)) > bufp->allocated) do { unsigned char *old_buffer = bufp->buffer; if (bufp->allocated == (1 << 16)) return REG_ESIZE; bufp->allocated <<= 1; if (bufp->allocated > (1 << 16)) bufp->allocated = (1 << 16); bufp->buffer = (unsigned char *) realloc ((bufp->buffer), (bufp->allocated)); if (bufp->buffer == ((void *)0)) return REG_ESPACE; if (old_buffer != bufp->buffer) { int incr = bufp->buffer - old_buffer; (b) += incr; (begalt) += incr; if (fixup_alt_jump) (fixup_alt_jump) += incr; if (laststart) (laststart) += incr; if (pending_exact) (pending_exact) += incr; } } while (0); *b++ = (unsigned char) (wordend); } while (0);
              break;
            case 'b':
       if (syntax & (((((((((((((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1))
  goto normal_char;
              do { while ((unsigned long) (b - bufp->buffer + (1)) > bufp->allocated) do { unsigned char *old_buffer = bufp->buffer; if (bufp->allocated == (1 << 16)) return REG_ESIZE; bufp->allocated <<= 1; if (bufp->allocated > (1 << 16)) bufp->allocated = (1 << 16); bufp->buffer = (unsigned char *) realloc ((bufp->buffer), (bufp->allocated)); if (bufp->buffer == ((void *)0)) return REG_ESPACE; if (old_buffer != bufp->buffer) { int incr = bufp->buffer - old_buffer; (b) += incr; (begalt) += incr; if (fixup_alt_jump) (fixup_alt_jump) += incr; if (laststart) (laststart) += incr; if (pending_exact) (pending_exact) += incr; } } while (0); *b++ = (unsigned char) (wordbound); } while (0);
              break;
            case 'B':
       if (syntax & (((((((((((((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1))
  goto normal_char;
              do { while ((unsigned long) (b - bufp->buffer + (1)) > bufp->allocated) do { unsigned char *old_buffer = bufp->buffer; if (bufp->allocated == (1 << 16)) return REG_ESIZE; bufp->allocated <<= 1; if (bufp->allocated > (1 << 16)) bufp->allocated = (1 << 16); bufp->buffer = (unsigned char *) realloc ((bufp->buffer), (bufp->allocated)); if (bufp->buffer == ((void *)0)) return REG_ESPACE; if (old_buffer != bufp->buffer) { int incr = bufp->buffer - old_buffer; (b) += incr; (begalt) += incr; if (fixup_alt_jump) (fixup_alt_jump) += incr; if (laststart) (laststart) += incr; if (pending_exact) (pending_exact) += incr; } } while (0); *b++ = (unsigned char) (notwordbound); } while (0);
              break;
            case 0:
       if (syntax & (((((((((((((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1))
  goto normal_char;
              do { while ((unsigned long) (b - bufp->buffer + (1)) > bufp->allocated) do { unsigned char *old_buffer = bufp->buffer; if (bufp->allocated == (1 << 16)) return REG_ESIZE; bufp->allocated <<= 1; if (bufp->allocated > (1 << 16)) bufp->allocated = (1 << 16); bufp->buffer = (unsigned char *) realloc ((bufp->buffer), (bufp->allocated)); if (bufp->buffer == ((void *)0)) return REG_ESPACE; if (old_buffer != bufp->buffer) { int incr = bufp->buffer - old_buffer; (b) += incr; (begalt) += incr; if (fixup_alt_jump) (fixup_alt_jump) += incr; if (laststart) (laststart) += incr; if (pending_exact) (pending_exact) += incr; } } while (0); *b++ = (unsigned char) (begbuf); } while (0);
              break;
            case 0:
       if (syntax & (((((((((((((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1))
  goto normal_char;
              do { while ((unsigned long) (b - bufp->buffer + (1)) > bufp->allocated) do { unsigned char *old_buffer = bufp->buffer; if (bufp->allocated == (1 << 16)) return REG_ESIZE; bufp->allocated <<= 1; if (bufp->allocated > (1 << 16)) bufp->allocated = (1 << 16); bufp->buffer = (unsigned char *) realloc ((bufp->buffer), (bufp->allocated)); if (bufp->buffer == ((void *)0)) return REG_ESPACE; if (old_buffer != bufp->buffer) { int incr = bufp->buffer - old_buffer; (b) += incr; (begalt) += incr; if (fixup_alt_jump) (fixup_alt_jump) += incr; if (laststart) (laststart) += incr; if (pending_exact) (pending_exact) += incr; } } while (0); *b++ = (unsigned char) (endbuf); } while (0);
              break;
            case '1': case '2': case '3': case '4': case '5':
            case '6': case '7': case '8': case '9':
              if (syntax & ((((((((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1))
                goto normal_char;
              c1 = c - '0';
              if (c1 > regnum)
                return (free (compile_stack.stack), REG_ESUBREG);
              if (group_in_compile_stack (compile_stack, (regnum_t) c1))
                goto normal_char;
              laststart = b;
              do { while ((unsigned long) (b - bufp->buffer + (2)) > bufp->allocated) do { unsigned char *old_buffer = bufp->buffer; if (bufp->allocated == (1 << 16)) return REG_ESIZE; bufp->allocated <<= 1; if (bufp->allocated > (1 << 16)) bufp->allocated = (1 << 16); bufp->buffer = (unsigned char *) realloc ((bufp->buffer), (bufp->allocated)); if (bufp->buffer == ((void *)0)) return REG_ESPACE; if (old_buffer != bufp->buffer) { int incr = bufp->buffer - old_buffer; (b) += incr; (begalt) += incr; if (fixup_alt_jump) (fixup_alt_jump) += incr; if (laststart) (laststart) += incr; if (pending_exact) (pending_exact) += incr; } } while (0); *b++ = (unsigned char) (duplicate); *b++ = (unsigned char) (c1); } while (0);
              break;
            case '+':
            case '?':
              if (syntax & (((unsigned long int) 1) << 1))
                goto handle_plus;
              else
                goto normal_backslash;
            default:
            normal_backslash:
              c = (translate ? (char) translate[(unsigned char) (c)] : (c));
              goto normal_char;
            }
          break;
 default:
 normal_char:
          if (!pending_exact
              || pending_exact + *pending_exact + 1 != b
       || *pending_exact == (1 << 8) - 1
              || *p == '*' || *p == '^'
       || ((syntax & (((unsigned long int) 1) << 1))
    ? *p == 'a' && (p[1] == '+' || p[1] == '?')
    : (*p == '+' || *p == '?'))
       || ((syntax & (((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1))
                  && ((syntax & ((((((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1))
        ? *p == 'a'
                      : (p[0] == 'a' && p[1] == 'a'))))
         {
            laststart = b;
            do { while ((unsigned long) (b - bufp->buffer + (2)) > bufp->allocated) do { unsigned char *old_buffer = bufp->buffer; if (bufp->allocated == (1 << 16)) return REG_ESIZE; bufp->allocated <<= 1; if (bufp->allocated > (1 << 16)) bufp->allocated = (1 << 16); bufp->buffer = (unsigned char *) realloc ((bufp->buffer), (bufp->allocated)); if (bufp->buffer == ((void *)0)) return REG_ESPACE; if (old_buffer != bufp->buffer) { int incr = bufp->buffer - old_buffer; (b) += incr; (begalt) += incr; if (fixup_alt_jump) (fixup_alt_jump) += incr; if (laststart) (laststart) += incr; if (pending_exact) (pending_exact) += incr; } } while (0); *b++ = (unsigned char) (exactn); *b++ = (unsigned char) (0); } while (0);
            pending_exact = b - 1;
          }
          do { while ((unsigned long) (b - bufp->buffer + (1)) > bufp->allocated) do { unsigned char *old_buffer = bufp->buffer; if (bufp->allocated == (1 << 16)) return REG_ESIZE; bufp->allocated <<= 1; if (bufp->allocated > (1 << 16)) bufp->allocated = (1 << 16); bufp->buffer = (unsigned char *) realloc ((bufp->buffer), (bufp->allocated)); if (bufp->buffer == ((void *)0)) return REG_ESPACE; if (old_buffer != bufp->buffer) { int incr = bufp->buffer - old_buffer; (b) += incr; (begalt) += incr; if (fixup_alt_jump) (fixup_alt_jump) += incr; if (laststart) (laststart) += incr; if (pending_exact) (pending_exact) += incr; } } while (0); *b++ = (unsigned char) (c); } while (0);
          (*pending_exact)++;
            break;
        }
  if (fixup_alt_jump)
    byte_store_op1 (jump_past_alt, fixup_alt_jump, (int) ((b) - (fixup_alt_jump) - (1 + 2)));
  if (!(compile_stack.avail == 0))
    return (free (compile_stack.stack), REG_EPAREN);
  if (syntax & ((((((((((((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1))
    do { while ((unsigned long) (b - bufp->buffer + (1)) > bufp->allocated) do { unsigned char *old_buffer = bufp->buffer; if (bufp->allocated == (1 << 16)) return REG_ESIZE; bufp->allocated <<= 1; if (bufp->allocated > (1 << 16)) bufp->allocated = (1 << 16); bufp->buffer = (unsigned char *) realloc ((bufp->buffer), (bufp->allocated)); if (bufp->buffer == ((void *)0)) return REG_ESPACE; if (old_buffer != bufp->buffer) { int incr = bufp->buffer - old_buffer; (b) += incr; (begalt) += incr; if (fixup_alt_jump) (fixup_alt_jump) += incr; if (laststart) (laststart) += incr; if (pending_exact) (pending_exact) += incr; } } while (0); *b++ = (unsigned char) (succeed); } while (0);
  free (compile_stack.stack);
  bufp->used = b - bufp->buffer;
  return REG_NOERROR;
}
static void
byte_store_op1 (op, loc, arg)
    re_opcode_t op;
    unsigned char *loc;
    int arg;
{
  *loc = (unsigned char) op;
  do { (loc + 1)[0] = (arg) & 0377; (loc + 1)[1] = (arg) >> 8; } while (0);
}
static void
byte_store_op2 (op, loc, arg1, arg2)
    re_opcode_t op;
    unsigned char *loc;
    int arg1, arg2;
{
  *loc = (unsigned char) op;
  do { (loc + 1)[0] = (arg1) & 0377; (loc + 1)[1] = (arg1) >> 8; } while (0);
  do { (loc + 1 + 2)[0] = (arg2) & 0377; (loc + 1 + 2)[1] = (arg2) >> 8; } while (0);
}
static void
byte_insert_op1 (op, loc, arg, end)
    re_opcode_t op;
    unsigned char *loc;
    int arg;
    unsigned char *end;
{
  register unsigned char *pfrom = end;
  register unsigned char *pto = end + 1 + 2;
  while (pfrom != loc)
    *--pto = *--pfrom;
  byte_store_op1 (op, loc, arg);
}
static void
byte_insert_op2 (op, loc, arg1, arg2, end)
    re_opcode_t op;
    unsigned char *loc;
    int arg1, arg2;
    unsigned char *end;
{
  register unsigned char *pfrom = end;
  register unsigned char *pto = end + 1 + 2 * 2;
  while (pfrom != loc)
    *--pto = *--pfrom;
  byte_store_op2 (op, loc, arg1, arg2);
}
static boolean
byte_at_begline_loc_p (pattern, p, syntax)
    const char *pattern, *p;
    reg_syntax_t syntax;
{
  const char *prev = p - 2;
  boolean prev_prev_backslash = prev > pattern && prev[-1] == 'a';
  return
       (*prev == '(' && (syntax & (((((((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) || prev_prev_backslash))
    || (*prev == '|' && (syntax & (((((((((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) || prev_prev_backslash));
}
static boolean
byte_at_endline_loc_p (p, pend, syntax)
    const char *p, *pend;
    reg_syntax_t syntax;
{
  const char *next = p;
  boolean next_backslash = *next == 'a';
  const char *next_next = p + 1 < pend ? p + 1 : 0;
  return
       (syntax & (((((((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) ? *next == ')'
        : next_backslash && next_next && *next_next == ')')
    || (syntax & (((((((((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) ? *next == '|'
        : next_backslash && next_next && *next_next == '|');
}
static reg_errcode_t
byte_compile_range (range_start_char, p_ptr, pend, translate, syntax, b)
     unsigned int range_start_char;
     const char *p_ptr, *pend;
     char * translate;
     reg_syntax_t syntax;
     unsigned char *b;
{
  unsigned this_char;
  const char *p = *p_ptr;
  reg_errcode_t ret;
  unsigned end_char;
  if (p == pend)
    return REG_ERANGE;
  (*p_ptr)++;
  ret = syntax & ((((((((((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) ? REG_ERANGE : REG_NOERROR;
  range_start_char = (translate ? (char) translate[(unsigned char) (range_start_char)] : (range_start_char));
  end_char = ((unsigned)(translate ? (char) translate[(unsigned char) (p[0])] : (p[0])) & ((1 << 8) - 1));
  for (this_char = range_start_char; this_char <= end_char; ++this_char)
    {
      (b[((unsigned char) ((translate ? (char) translate[(unsigned char) (this_char)] : (this_char)))) / 8] |= 1 << (((unsigned char) (translate ? (char) translate[(unsigned char) (this_char)] : (this_char))) % 8));
      ret = REG_NOERROR;
    }
  return ret;
}
static int
byte_re_compile_fastmap (bufp)
     struct re_pattern_buffer *bufp;
{
  int j, k;
  byte_fail_stack_type fail_stack;
  char *destination;
  register char *fastmap = bufp->fastmap;
  unsigned char *pattern = bufp->buffer;
  register unsigned char *pend = pattern + bufp->used;
  unsigned char *p = pattern;
  boolean path_can_be_null = 1;
  boolean succeed_n_p = 0;
  ;
  do { fail_stack.stack = (byte_fail_stack_elt_t *) alloca (5 * sizeof (byte_fail_stack_elt_t)); if (fail_stack.stack == ((void *)0)) return -2; fail_stack.size = 5; fail_stack.avail = 0; } while (0);
  (memset (fastmap, 0, 1 << 8), (fastmap));
  bufp->fastmap_accurate = 1;
  bufp->can_be_null = 0;
  while (1)
    {
      if (p == pend || *p == (unsigned char) succeed)
 {
   if (!(fail_stack.avail == 0))
     {
       bufp->can_be_null |= path_can_be_null;
       path_can_be_null = 1;
       p = fail_stack.stack[--fail_stack.avail].pointer;
       continue;
     }
   else
     break;
 }
      ;
      switch (((re_opcode_t) *p++))
 {
 case duplicate:
   bufp->can_be_null = 1;
          goto done;
 case exactn:
          fastmap[p[1]] = 1;
   break;
        case charset:
          for (j = *p++ * 8 - 1; j >= 0; j--)
     if (p[j / 8] & (1 << (j % 8)))
              fastmap[j] = 1;
   break;
 case charset_not:
   for (j = *p * 8; j < (1 << 8); j++)
            fastmap[j] = 1;
   for (j = *p++ * 8 - 1; j >= 0; j--)
     if (!(p[j / 8] & (1 << (j % 8))))
              fastmap[j] = 1;
          break;
 case wordchar:
   for (j = 0; j < (1 << 8); j++)
     if (re_syntax_table[(unsigned char) (j)] == 1)
       fastmap[j] = 1;
   break;
 case notwordchar:
   for (j = 0; j < (1 << 8); j++)
     if (re_syntax_table[(unsigned char) (j)] != 1)
       fastmap[j] = 1;
   break;
        case anychar:
   {
     int fastmap_newline = fastmap['\n'];
     for (j = 0; j < (1 << 8); j++)
       fastmap[j] = 1;
     if (!(bufp->syntax & ((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1)))
       fastmap['\n'] = fastmap_newline;
     else if (bufp->can_be_null)
       goto done;
     break;
   }
        case no_op:
        case begline:
        case endline:
 case begbuf:
 case endbuf:
 case wordbound:
 case notwordbound:
 case wordbeg:
 case wordend:
        case push_dummy_failure:
          continue;
 case jump_n:
        case pop_failure_jump:
 case maybe_pop_jump:
 case jump:
        case jump_past_alt:
 case dummy_failure_jump:
          do { do { (j) = *(p) & 0377; (j) += ((signed char) (*((p) + 1))) << 8; } while (0); (p) += 2; } while (0);
   p += j;
   if (j > 0)
     continue;
          if ((re_opcode_t) *p != on_failure_jump
       && (re_opcode_t) *p != succeed_n)
     continue;
          p++;
          do { do { (j) = *(p) & 0377; (j) += ((signed char) (*((p) + 1))) << 8; } while (0); (p) += 2; } while (0);
          p += j;
          if (!(fail_stack.avail == 0)
       && fail_stack.stack[fail_stack.avail - 1].pointer == p)
            fail_stack.avail--;
          continue;
        case on_failure_jump:
        case on_failure_keep_string_jump:
 handle_on_failure_jump:
          do { do { (j) = *(p) & 0377; (j) += ((signed char) (*((p) + 1))) << 8; } while (0); (p) += 2; } while (0);
          if (p + j < pend)
            {
              if (!(((fail_stack.avail == fail_stack.size) && !((fail_stack).size > (unsigned) (xre_max_failures * (5 * 3 + 4)) ? 0 : ((fail_stack).stack = (byte_fail_stack_elt_t *) (destination = (char *) alloca (((fail_stack).size << 1) * sizeof (byte_fail_stack_elt_t)), memcpy (destination, (fail_stack).stack, (fail_stack).size * sizeof (byte_fail_stack_elt_t))), (fail_stack).stack == ((void *)0) ? 0 : ((fail_stack).size <<= 1, 1)))) ? 0 : ((fail_stack).stack[(fail_stack).avail++].pointer = p + j, 1)))
  {
    ;
    return -2;
  }
            }
          else
            bufp->can_be_null = 1;
          if (succeed_n_p)
            {
              do { do { (k) = *(p) & 0377; (k) += ((signed char) (*((p) + 1))) << 8; } while (0); (p) += 2; } while (0);
              succeed_n_p = 0;
     }
          continue;
 case succeed_n:
          p += 2;
          do { do { (k) = *(p) & 0377; (k) += ((signed char) (*((p) + 1))) << 8; } while (0); (p) += 2; } while (0);
          if (k == 0)
     {
              p -= 2 * 2;
         succeed_n_p = 1;
              goto handle_on_failure_jump;
            }
          continue;
 case set_number_at:
          p += 2 * 2;
          continue;
 case start_memory:
        case stop_memory:
   p += 2;
   continue;
 default:
          abort ();
        }
      path_can_be_null = 0;
      p = pend;
    }
  bufp->can_be_null |= path_can_be_null;
 done:
  ;
  return 0;
}
static int
byte_re_search_2 (bufp, string1, size1, string2, size2, startpos, range,
       regs, stop)
     struct re_pattern_buffer *bufp;
     const char *string1, *string2;
     int size1, size2;
     int startpos;
     int range;
     struct re_registers *regs;
     int stop;
{
  int val;
  register char *fastmap = bufp->fastmap;
  register char * translate = bufp->translate;
  int total_size = size1 + size2;
  int endpos = startpos + range;
  if (startpos < 0 || startpos > total_size)
    return -1;
  if (endpos < 0)
    range = 0 - startpos;
  else if (endpos > total_size)
    range = total_size - startpos;
  if (bufp->used > 0 && range > 0
      && ((re_opcode_t) bufp->buffer[0] == begbuf
   || ((re_opcode_t) bufp->buffer[0] == begline
       && !bufp->newline_anchor)))
    {
      if (startpos > 0)
 return -1;
      else
 range = 1;
    }
  if (fastmap && !bufp->fastmap_accurate)
    if (xre_compile_fastmap (bufp) == -2)
      return -2;
  for (;;)
    {
      if (fastmap && startpos < total_size && !bufp->can_be_null)
 {
   if (range > 0)
     {
       register const char *d;
       register int lim = 0;
       int irange = range;
              if (startpos < size1 && startpos + range >= size1)
                lim = range - (size1 - startpos);
       d = (startpos >= size1 ? string2 - size1 : string1) + startpos;
       if (translate)
                while (range > lim
                       && !fastmap[(unsigned char)
       translate[(unsigned char) *d++]])
                  range--;
       else
                while (range > lim && !fastmap[(unsigned char) *d++])
                  range--;
       startpos += irange - range;
     }
   else
     {
       register char c = (size1 == 0 || startpos >= size1
          ? string2[startpos - size1]
          : string1[startpos]);
       if (!fastmap[(unsigned char) (translate ? (char) translate[(unsigned char) (c)] : (c))])
  goto advance;
     }
 }
      if (range >= 0 && startpos == total_size && fastmap
          && !bufp->can_be_null)
       {
         return -1;
       }
      val = byte_re_match_2_internal (bufp, string1, size1, string2,
          size2, startpos, regs, stop);
      if (val >= 0)
 {
   return startpos;
 }
      if (val == -2)
 {
   return -2;
 }
    advance:
      if (!range)
        break;
      else if (range > 0)
        {
          range--;
          startpos++;
        }
      else
        {
          range++;
          startpos--;
        }
    }
  return -1;
}
static boolean byte_group_match_null_string_p (unsigned char *p, unsigned char *end, byte_register_info_type *reg_info)
                                           ;
static boolean byte_alt_match_null_string_p (unsigned char *p, unsigned char *end, byte_register_info_type *reg_info)
                                           ;
static boolean byte_common_op_match_null_string_p (unsigned char *p, unsigned char *end, byte_register_info_type *reg_info)
                                           ;
static int byte_bcmp_translate (const char *s1, const char *s2, int len, char *translate)
                                   ;
static int
byte_re_match_2_internal (bufp, string1, size1,string2, size2, pos,
     regs, stop)
     struct re_pattern_buffer *bufp;
     const char *string1, *string2;
     int size1, size2;
     int pos;
     struct re_registers *regs;
     int stop;
{
  int mcnt;
  unsigned char *p1;
  const char *end1, *end2;
  const char *end_match_1, *end_match_2;
  const char *d, *dend;
  unsigned char *p = bufp->buffer;
  register unsigned char *pend = p + bufp->used;
  unsigned char *just_past_start_mem = 0;
  char * translate = bufp->translate;
  byte_fail_stack_type fail_stack;
  size_t num_regs = bufp->re_nsub + 1;
  active_reg_t lowest_active_reg = ((1 << 8) + 1);
  active_reg_t highest_active_reg = (1 << 8);
  const char *regstart, *regend;
  const char *old_regstart, *old_regend;
  byte_register_info_type *reg_info;
  unsigned best_regs_set = 0;
  const char *best_regstart, *best_regend;
  const char *match_end = ((void *)0);
  int set_regs_matched_done = 0;
  const char *reg_dummy;
  byte_register_info_type *reg_info_dummy;
  ;
  do { fail_stack.stack = (byte_fail_stack_elt_t *) alloca (5 * sizeof (byte_fail_stack_elt_t)); if (fail_stack.stack == ((void *)0)) return -2; fail_stack.size = 5; fail_stack.avail = 0; } while (0);
  if (bufp->re_nsub)
    {
      regstart = ((const char * *) alloca ((num_regs) * sizeof (const char *)));
      regend = ((const char * *) alloca ((num_regs) * sizeof (const char *)));
      old_regstart = ((const char * *) alloca ((num_regs) * sizeof (const char *)));
      old_regend = ((const char * *) alloca ((num_regs) * sizeof (const char *)));
      best_regstart = ((const char * *) alloca ((num_regs) * sizeof (const char *)));
      best_regend = ((const char * *) alloca ((num_regs) * sizeof (const char *)));
      reg_info = ((byte_register_info_type *) alloca ((num_regs) * sizeof (byte_register_info_type)));
      reg_dummy = ((const char * *) alloca ((num_regs) * sizeof (const char *)));
      reg_info_dummy = ((byte_register_info_type *) alloca ((num_regs) * sizeof (byte_register_info_type)));
      if (!(regstart && regend && old_regstart && old_regend && reg_info
            && best_regstart && best_regend && reg_dummy && reg_info_dummy))
        {
          do { ; if (regstart) ((void)0); regstart = ((void *)0); if (regend) ((void)0); regend = ((void *)0); if (old_regstart) ((void)0); old_regstart = ((void *)0); if (old_regend) ((void)0); old_regend = ((void *)0); if (best_regstart) ((void)0); best_regstart = ((void *)0); if (best_regend) ((void)0); best_regend = ((void *)0); if (reg_info) ((void)0); reg_info = ((void *)0); if (reg_dummy) ((void)0); reg_dummy = ((void *)0); if (reg_info_dummy) ((void)0); reg_info_dummy = ((void *)0); } while (0);
          return -2;
        }
    }
  else
    {
      regstart = regend = old_regstart = old_regend = best_regstart
        = best_regend = reg_dummy = ((void *)0);
      reg_info = reg_info_dummy = (byte_register_info_type *) ((void *)0);
    }
  if (pos < 0 || pos > size1 + size2)
    {
      do { ; if (regstart) ((void)0); regstart = ((void *)0); if (regend) ((void)0); regend = ((void *)0); if (old_regstart) ((void)0); old_regstart = ((void *)0); if (old_regend) ((void)0); old_regend = ((void *)0); if (best_regstart) ((void)0); best_regstart = ((void *)0); if (best_regend) ((void)0); best_regend = ((void *)0); if (reg_info) ((void)0); reg_info = ((void *)0); if (reg_dummy) ((void)0); reg_dummy = ((void *)0); if (reg_info_dummy) ((void)0); reg_info_dummy = ((void *)0); } while (0);
      return -1;
    }
  for (mcnt = 1; (unsigned) mcnt < num_regs; mcnt++)
    {
      regstart[mcnt] = regend[mcnt]
        = old_regstart[mcnt] = old_regend[mcnt] = (&byte_reg_unset_dummy);
      ((reg_info[mcnt]).bits.match_null_string_p) = 3;
      ((reg_info[mcnt]).bits.is_active) = 0;
      ((reg_info[mcnt]).bits.matched_something) = 0;
      ((reg_info[mcnt]).bits.ever_matched_something) = 0;
    }
  if (size2 == 0 && string1 != ((void *)0))
    {
      string2 = string1;
      size2 = size1;
      string1 = 0;
      size1 = 0;
    }
  end1 = string1 + size1;
  end2 = string2 + size2;
  if (stop <= size1)
    {
      end_match_1 = string1 + stop;
      end_match_2 = string2;
    }
  else
    {
      end_match_1 = end1;
      end_match_2 = string2 + stop - size1;
    }
  if (size1 > 0 && pos <= size1)
    {
      d = string1 + pos;
      dend = end_match_1;
    }
  else
    {
      d = string2 + pos - size1;
      dend = end_match_2;
    }
  ;
  ;
  ;
  ;
  ;
  for (;;)
    {
      ;
      if (p == pend)
 {
          ;
          if (d != end_match_2)
     {
       boolean same_str_p = ((size1 && string1 <= (match_end) && (match_end) <= string1 + size1)
        == (dend == end_match_1));
       boolean best_match_p;
       if (same_str_p)
  best_match_p = d > match_end;
       else
  best_match_p = !(dend == end_match_1);
              ;
              if (!(fail_stack.avail == 0))
                {
                  if (!best_regs_set || best_match_p)
                    {
                      best_regs_set = 1;
                      match_end = d;
                      ;
                      for (mcnt = 1; (unsigned) mcnt < num_regs; mcnt++)
                        {
                          best_regstart[mcnt] = regstart[mcnt];
                          best_regend[mcnt] = regend[mcnt];
                        }
                    }
                  goto fail;
                }
              else if (best_regs_set && !best_match_p)
                {
           restore_best_regs:
                  ;
                  d = match_end;
                  dend = ((d >= string1 && d <= end1)
             ? end_match_1 : end_match_2);
    for (mcnt = 1; (unsigned) mcnt < num_regs; mcnt++)
      {
        regstart[mcnt] = best_regstart[mcnt];
        regend[mcnt] = best_regend[mcnt];
      }
                }
            }
 succeed_label:
          ;
          if (regs && !bufp->no_sub)
     {
              if (bufp->regs_allocated == 0)
                {
                  regs->num_regs = ((30) > (num_regs + 1) ? (30) : (num_regs + 1));
                  regs->start = ((regoff_t *) malloc ((regs->num_regs) * sizeof (regoff_t)));
                  regs->end = ((regoff_t *) malloc ((regs->num_regs) * sizeof (regoff_t)));
                  if (regs->start == ((void *)0) || regs->end == ((void *)0))
      {
        do { ; if (regstart) ((void)0); regstart = ((void *)0); if (regend) ((void)0); regend = ((void *)0); if (old_regstart) ((void)0); old_regstart = ((void *)0); if (old_regend) ((void)0); old_regend = ((void *)0); if (best_regstart) ((void)0); best_regstart = ((void *)0); if (best_regend) ((void)0); best_regend = ((void *)0); if (reg_info) ((void)0); reg_info = ((void *)0); if (reg_dummy) ((void)0); reg_dummy = ((void *)0); if (reg_info_dummy) ((void)0); reg_info_dummy = ((void *)0); } while (0);
        return -2;
      }
                  bufp->regs_allocated = 1;
                }
              else if (bufp->regs_allocated == 1)
                {
                  if (regs->num_regs < num_regs + 1)
                    {
                      regs->num_regs = num_regs + 1;
                      ((regs->start) = (regoff_t *) realloc (regs->start, (regs->num_regs) * sizeof (regoff_t)));
                      ((regs->end) = (regoff_t *) realloc (regs->end, (regs->num_regs) * sizeof (regoff_t)));
                      if (regs->start == ((void *)0) || regs->end == ((void *)0))
   {
     do { ; if (regstart) ((void)0); regstart = ((void *)0); if (regend) ((void)0); regend = ((void *)0); if (old_regstart) ((void)0); old_regstart = ((void *)0); if (old_regend) ((void)0); old_regend = ((void *)0); if (best_regstart) ((void)0); best_regstart = ((void *)0); if (best_regend) ((void)0); best_regend = ((void *)0); if (reg_info) ((void)0); reg_info = ((void *)0); if (reg_dummy) ((void)0); reg_dummy = ((void *)0); if (reg_info_dummy) ((void)0); reg_info_dummy = ((void *)0); } while (0);
     return -2;
   }
                    }
                }
              else
  {
    ;
  }
              if (regs->num_regs > 0)
                {
                  regs->start[0] = pos;
                  regs->end[0] = ((dend == end_match_1)
      ? ((regoff_t) (d - string1))
             : ((regoff_t) (d - string2 + size1)));
                }
       for (mcnt = 1; (unsigned) mcnt < ((num_regs) < (regs->num_regs) ? (num_regs) : (regs->num_regs));
     mcnt++)
  {
                  if (((regstart[mcnt]) == (&byte_reg_unset_dummy)) || ((regend[mcnt]) == (&byte_reg_unset_dummy)))
                    regs->start[mcnt] = regs->end[mcnt] = -1;
                  else
                    {
        regs->start[mcnt]
   = (regoff_t) ((size1 && string1 <= (regstart[mcnt]) && (regstart[mcnt]) <= string1 + size1) ? ((regoff_t) ((regstart[mcnt]) - string1)) : ((regoff_t) ((regstart[mcnt]) - string2 + size1)));
                      regs->end[mcnt]
   = (regoff_t) ((size1 && string1 <= (regend[mcnt]) && (regend[mcnt]) <= string1 + size1) ? ((regoff_t) ((regend[mcnt]) - string1)) : ((regoff_t) ((regend[mcnt]) - string2 + size1)));
                    }
  }
              for (mcnt = num_regs; (unsigned) mcnt < regs->num_regs; mcnt++)
                regs->start[mcnt] = regs->end[mcnt] = -1;
     }
         
                                                                        ;
          ;
          mcnt = d - pos - ((dend == end_match_1)
       ? string1
       : string2 - size1);
          ;
          do { ; if (regstart) ((void)0); regstart = ((void *)0); if (regend) ((void)0); regend = ((void *)0); if (old_regstart) ((void)0); old_regstart = ((void *)0); if (old_regend) ((void)0); old_regend = ((void *)0); if (best_regstart) ((void)0); best_regstart = ((void *)0); if (best_regend) ((void)0); best_regend = ((void *)0); if (reg_info) ((void)0); reg_info = ((void *)0); if (reg_dummy) ((void)0); reg_dummy = ((void *)0); if (reg_info_dummy) ((void)0); reg_info_dummy = ((void *)0); } while (0);
          return mcnt;
        }
      switch (((re_opcode_t) *p++))
 {
        case no_op:
          ;
          break;
 case succeed:
          ;
   goto succeed_label;
 case exactn:
   mcnt = *p++;
          ;
          if (translate)
     {
       do
  {
    while (d == dend) { if (dend == end_match_2) goto fail; d = string2; dend = end_match_2; };
    if ((unsigned char) translate[(unsigned char) *d++]
        != (unsigned char) *p++)
                    goto fail;
  }
       while (--mcnt);
     }
   else
     {
       do
  {
    while (d == dend) { if (dend == end_match_2) goto fail; d = string2; dend = end_match_2; };
    if (*d++ != (char) *p++) goto fail;
  }
       while (--mcnt);
     }
   do { if (!set_regs_matched_done) { active_reg_t r; set_regs_matched_done = 1; for (r = lowest_active_reg; r <= highest_active_reg; r++) { ((reg_info[r]).bits.matched_something) = ((reg_info[r]).bits.ever_matched_something) = 1; } } } while (0);
          break;
 case anychar:
          ;
          while (d == dend) { if (dend == end_match_2) goto fail; d = string2; dend = end_match_2; };
          if ((!(bufp->syntax & ((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1)) && (translate ? (char) translate[(unsigned char) (*d)] : (*d)) == '\n')
              || (bufp->syntax & (((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) && (translate ? (char) translate[(unsigned char) (*d)] : (*d)) == 0))
     goto fail;
          do { if (!set_regs_matched_done) { active_reg_t r; set_regs_matched_done = 1; for (r = lowest_active_reg; r <= highest_active_reg; r++) { ((reg_info[r]).bits.matched_something) = ((reg_info[r]).bits.ever_matched_something) = 1; } } } while (0);
          ;
          d++;
   break;
 case charset:
 case charset_not:
   {
     register unsigned char c;
     boolean not = (re_opcode_t) *(p - 1) == charset_not;
            ;
     while (d == dend) { if (dend == end_match_2) goto fail; d = string2; dend = end_match_2; };
     c = (translate ? (char) translate[(unsigned char) (*d)] : (*d));
     if (c < (unsigned) (*p * 8)
  && p[1 + c / 8] & (1 << (c % 8)))
       not = !not;
     p += 1 + *p;
     if (!not) goto fail;
     do { if (!set_regs_matched_done) { active_reg_t r; set_regs_matched_done = 1; for (r = lowest_active_reg; r <= highest_active_reg; r++) { ((reg_info[r]).bits.matched_something) = ((reg_info[r]).bits.ever_matched_something) = 1; } } } while (0);
            d++;
     break;
   }
        case start_memory:
  
                                  ;
   p1 = p;
          if (((reg_info[*p]).bits.match_null_string_p) == 3)
            ((reg_info[*p]).bits.match_null_string_p)
              = byte_group_match_null_string_p (&p1, pend, reg_info);
          old_regstart[*p] = ((reg_info[*p]).bits.match_null_string_p)
                             ? ((regstart[*p]) == (&byte_reg_unset_dummy))
                             : regstart[*p];
  
                                         ;
          regstart[*p] = d;
   ;
          ((reg_info[*p]).bits.is_active) = 1;
          ((reg_info[*p]).bits.matched_something) = 0;
   set_regs_matched_done = 0;
          highest_active_reg = *p;
          if (lowest_active_reg == ((1 << 8) + 1))
            lowest_active_reg = *p;
          p += 2;
   just_past_start_mem = p;
          break;
 case stop_memory:
  
                                  ;
          old_regend[*p] = ((reg_info[*p]).bits.match_null_string_p)
                           ? ((regend[*p]) == (&byte_reg_unset_dummy)) : regend[*p];
  
                                       ;
          regend[*p] = d;
   ;
          ((reg_info[*p]).bits.is_active) = 0;
   set_regs_matched_done = 0;
          if (lowest_active_reg == highest_active_reg)
            {
              lowest_active_reg = ((1 << 8) + 1);
              highest_active_reg = (1 << 8);
            }
          else
            {
              unsigned char r = *p - 1;
              while (r > 0 && !((reg_info[r]).bits.is_active))
                r--;
       if (r == 0)
                {
                  lowest_active_reg = ((1 << 8) + 1);
                  highest_active_reg = (1 << 8);
                }
              else
                highest_active_reg = r;
            }
          if ((!((reg_info[*p]).bits.matched_something)
               || just_past_start_mem == p - 1)
       && (p + 2) < pend)
            {
              boolean is_a_jump_n = 0;
              p1 = p + 2;
              mcnt = 0;
              switch ((re_opcode_t) *p1++)
                {
                  case jump_n:
      is_a_jump_n = 1;
                  case pop_failure_jump:
    case maybe_pop_jump:
    case jump:
    case dummy_failure_jump:
                    do { do { (mcnt) = *(p1) & 0377; (mcnt) += ((signed char) (*((p1) + 1))) << 8; } while (0); (p1) += 2; } while (0);
      if (is_a_jump_n)
        p1 += 2;
                    break;
                  default:
                                     ;
                }
       p1 += mcnt;
              if (mcnt < 0 && (re_opcode_t) *p1 == on_failure_jump
                  && (re_opcode_t) p1[1+2] == start_memory
    && p1[2+2] == *p)
  {
                  if (((reg_info[*p]).bits.ever_matched_something))
      {
        unsigned r;
                      ((reg_info[*p]).bits.ever_matched_something) = 0;
                      for (r = *p; r < (unsigned) *p + (unsigned) *(p + 1);
      r++)
                        {
                          regstart[r] = old_regstart[r];
                          if (old_regend[r] >= regstart[r])
                            regend[r] = old_regend[r];
                        }
                    }
    p1++;
                  do { do { (mcnt) = *(p1) & 0377; (mcnt) += ((signed char) (*((p1) + 1))) << 8; } while (0); (p1) += 2; } while (0);
                  do { char *destination; active_reg_t this_reg; ; ; ; ; ; ; ; while (((fail_stack).size - (fail_stack).avail) < (((0 ? 0 : highest_active_reg - lowest_active_reg + 1) * 3) + 4)) { if (!((fail_stack).size > (unsigned) (xre_max_failures * (5 * 3 + 4)) ? 0 : ((fail_stack).stack = (byte_fail_stack_elt_t *) (destination = (char *) alloca (((fail_stack).size << 1) * sizeof (byte_fail_stack_elt_t)), memcpy (destination, (fail_stack).stack, (fail_stack).size * sizeof (byte_fail_stack_elt_t))), (fail_stack).stack == ((void *)0) ? 0 : ((fail_stack).size <<= 1, 1)))) return -2; ; ; } ; if (1) for (this_reg = lowest_active_reg; this_reg <= highest_active_reg; this_reg++) { ; ; ; fail_stack.stack[fail_stack.avail++].pointer = (unsigned char *) (regstart[this_reg]); ; fail_stack.stack[fail_stack.avail++].pointer = (unsigned char *) (regend[this_reg]); ; ; ; ; ; ; fail_stack.stack[fail_stack.avail++] = (reg_info[this_reg].word); } ; fail_stack.stack[fail_stack.avail++].integer = (lowest_active_reg); ; fail_stack.stack[fail_stack.avail++].integer = (highest_active_reg); ; ; fail_stack.stack[fail_stack.avail++].pointer = (unsigned char *) (p1 + mcnt); ; ; ; fail_stack.stack[fail_stack.avail++].pointer = (unsigned char *) (d); ; ; } while (0);
                  goto fail;
                }
            }
          p += 2;
          break;
        case duplicate:
   {
     register const char *d2, *dend2;
     int regno = *p++;
     ;
            if (((regstart[regno]) == (&byte_reg_unset_dummy)) || ((regend[regno]) == (&byte_reg_unset_dummy)))
              goto fail;
            d2 = regstart[regno];
            dend2 = (((size1 && string1 <= (regstart[regno]) && (regstart[regno]) <= string1 + size1)
        == (size1 && string1 <= (regend[regno]) && (regend[regno]) <= string1 + size1))
       ? regend[regno] : end_match_1);
     for (;;)
       {
  while (d2 == dend2)
    {
      if (dend2 == end_match_2) break;
      if (dend2 == regend[regno]) break;
                    d2 = string2;
                    dend2 = regend[regno];
    }
  if (d2 == dend2) break;
  while (d == dend) { if (dend == end_match_2) goto fail; d = string2; dend = end_match_2; };
  mcnt = dend - d;
                if (mcnt > dend2 - d2)
    mcnt = dend2 - d2;
  if (translate
                    ? byte_bcmp_translate (d, d2, mcnt, translate)
                    : memcmp (d, d2, mcnt*sizeof(unsigned char)))
    goto fail;
  d += mcnt, d2 += mcnt;
  do { if (!set_regs_matched_done) { active_reg_t r; set_regs_matched_done = 1; for (r = lowest_active_reg; r <= highest_active_reg; r++) { ((reg_info[r]).bits.matched_something) = ((reg_info[r]).bits.ever_matched_something) = 1; } } } while (0);
       }
   }
   break;
 case begline:
          ;
          if (((d) == (size1 ? string1 : string2) || !size2))
            {
              if (!bufp->not_bol) break;
            }
          else if (d[-1] == '\n' && bufp->newline_anchor)
            {
              break;
            }
          goto fail;
 case endline:
          ;
          if (((d) == end2))
            {
              if (!bufp->not_eol) break;
            }
          else if ((d == end1 ? *string2 : *d) == '\n'
                   && bufp->newline_anchor)
            {
              break;
            }
          goto fail;
        case begbuf:
          ;
          if (((d) == (size1 ? string1 : string2) || !size2))
            break;
          goto fail;
        case endbuf:
          ;
   if (((d) == end2))
     break;
          goto fail;
        case on_failure_keep_string_jump:
          ;
          do { do { (mcnt) = *(p) & 0377; (mcnt) += ((signed char) (*((p) + 1))) << 8; } while (0); (p) += 2; } while (0);
          ;
          do { char *destination; active_reg_t this_reg; ; ; ; ; ; ; ; while (((fail_stack).size - (fail_stack).avail) < (((0 ? 0 : highest_active_reg - lowest_active_reg + 1) * 3) + 4)) { if (!((fail_stack).size > (unsigned) (xre_max_failures * (5 * 3 + 4)) ? 0 : ((fail_stack).stack = (byte_fail_stack_elt_t *) (destination = (char *) alloca (((fail_stack).size << 1) * sizeof (byte_fail_stack_elt_t)), memcpy (destination, (fail_stack).stack, (fail_stack).size * sizeof (byte_fail_stack_elt_t))), (fail_stack).stack == ((void *)0) ? 0 : ((fail_stack).size <<= 1, 1)))) return -2; ; ; } ; if (1) for (this_reg = lowest_active_reg; this_reg <= highest_active_reg; this_reg++) { ; ; ; fail_stack.stack[fail_stack.avail++].pointer = (unsigned char *) (regstart[this_reg]); ; fail_stack.stack[fail_stack.avail++].pointer = (unsigned char *) (regend[this_reg]); ; ; ; ; ; ; fail_stack.stack[fail_stack.avail++] = (reg_info[this_reg].word); } ; fail_stack.stack[fail_stack.avail++].integer = (lowest_active_reg); ; fail_stack.stack[fail_stack.avail++].integer = (highest_active_reg); ; ; fail_stack.stack[fail_stack.avail++].pointer = (unsigned char *) (p + mcnt); ; ; ; fail_stack.stack[fail_stack.avail++].pointer = (unsigned char *) (((void *)0)); ; ; } while (0);
          break;
 case on_failure_jump:
        on_failure:
          ;
          do { do { (mcnt) = *(p) & 0377; (mcnt) += ((signed char) (*((p) + 1))) << 8; } while (0); (p) += 2; } while (0);
          ;
          p1 = p;
          while (p1 < pend && (re_opcode_t) *p1 == no_op)
            p1++;
          if (p1 < pend && (re_opcode_t) *p1 == start_memory)
            {
              highest_active_reg = *(p1 + 1) + *(p1 + 2);
              if (lowest_active_reg == ((1 << 8) + 1))
                lowest_active_reg = *(p1 + 1);
            }
          ;
          do { char *destination; active_reg_t this_reg; ; ; ; ; ; ; ; while (((fail_stack).size - (fail_stack).avail) < (((0 ? 0 : highest_active_reg - lowest_active_reg + 1) * 3) + 4)) { if (!((fail_stack).size > (unsigned) (xre_max_failures * (5 * 3 + 4)) ? 0 : ((fail_stack).stack = (byte_fail_stack_elt_t *) (destination = (char *) alloca (((fail_stack).size << 1) * sizeof (byte_fail_stack_elt_t)), memcpy (destination, (fail_stack).stack, (fail_stack).size * sizeof (byte_fail_stack_elt_t))), (fail_stack).stack == ((void *)0) ? 0 : ((fail_stack).size <<= 1, 1)))) return -2; ; ; } ; if (1) for (this_reg = lowest_active_reg; this_reg <= highest_active_reg; this_reg++) { ; ; ; fail_stack.stack[fail_stack.avail++].pointer = (unsigned char *) (regstart[this_reg]); ; fail_stack.stack[fail_stack.avail++].pointer = (unsigned char *) (regend[this_reg]); ; ; ; ; ; ; fail_stack.stack[fail_stack.avail++] = (reg_info[this_reg].word); } ; fail_stack.stack[fail_stack.avail++].integer = (lowest_active_reg); ; fail_stack.stack[fail_stack.avail++].integer = (highest_active_reg); ; ; fail_stack.stack[fail_stack.avail++].pointer = (unsigned char *) (p + mcnt); ; ; ; fail_stack.stack[fail_stack.avail++].pointer = (unsigned char *) (d); ; ; } while (0);
          break;
        case maybe_pop_jump:
          do { do { (mcnt) = *(p) & 0377; (mcnt) += ((signed char) (*((p) + 1))) << 8; } while (0); (p) += 2; } while (0);
          ;
          {
     register unsigned char *p2 = p;
     while (1)
       {
  if (p2 + 2 < pend
      && ((re_opcode_t) *p2 == stop_memory
   || (re_opcode_t) *p2 == start_memory))
    p2 += 3;
  else if (p2 + 2 + 2 * 2 < pend
    && (re_opcode_t) *p2 == dummy_failure_jump)
    p2 += 2 + 2 * 2;
  else
    break;
       }
     p1 = p + mcnt;
            if (p2 == pend)
       {
           p[-(1+2)] = (unsigned char)
    pop_failure_jump;
               
                                                                       ;
              }
            else if ((re_opcode_t) *p2 == exactn
       || (bufp->newline_anchor && (re_opcode_t) *p2 == endline))
       {
  register unsigned char c
                  = *p2 == (unsigned char) endline ? '\n' : p2[2];
                if (((re_opcode_t) p1[1+2] == exactn
      ) && p1[3+2] != c)
                  {
        p[-(1+2)] = (unsigned char)
        pop_failure_jump;
       
                                         ;
                  }
  else if ((re_opcode_t) p1[3] == charset
    || (re_opcode_t) p1[3] == charset_not)
    {
      int not = (re_opcode_t) p1[3] == charset_not;
      if (c < (unsigned) (p1[4] * 8)
   && p1[5 + c / 8] & (1 << (c % 8)))
        not = !not;
      if (!not)
                      {
            p[-3] = (unsigned char) pop_failure_jump;
                        ;
                      }
    }
       }
            else if ((re_opcode_t) *p2 == charset)
       {
                if ((re_opcode_t) p1[3] == exactn
       && ! ((int) p2[1] * 8 > (int) p1[5]
      && (p2[2 + p1[5] / 8]
          & (1 << (p1[5] % 8)))))
    {
      p[-3] = (unsigned char) pop_failure_jump;
      ;
                  }
  else if ((re_opcode_t) p1[3] == charset_not)
    {
      int idx;
      for (idx = 0; idx < (int) p2[1]; idx++)
        if (! (p2[2 + idx] == 0
        || (idx < (int) p1[4]
     && ((p2[2 + idx] & ~ p1[5 + idx]) == 0))))
   break;
      if (idx == p2[1])
                      {
            p[-3] = (unsigned char) pop_failure_jump;
                        ;
                      }
    }
  else if ((re_opcode_t) p1[3] == charset)
    {
      int idx;
      for (idx = 0;
    idx < (int) p2[1] && idx < (int) p1[4];
    idx++)
        if ((p2[2 + idx] & p1[5 + idx]) != 0)
   break;
      if (idx == p2[1] || idx == p1[4])
                      {
            p[-3] = (unsigned char) pop_failure_jump;
                        ;
                      }
    }
       }
   }
   p -= 2;
   if ((re_opcode_t) p[-1] != pop_failure_jump)
     {
       p[-1] = (unsigned char) jump;
              ;
       goto unconditional_jump;
     }
        case pop_failure_jump:
          {
            active_reg_t dummy_low_reg, dummy_high_reg;
            unsigned char *pdummy = ((void *)0);
            const char *sdummy = ((void *)0);
            ;
            { active_reg_t this_reg; const unsigned char *string_temp; ; ; ; ; ; ; ; string_temp = fail_stack.stack[--fail_stack.avail].pointer; if (string_temp != ((void *)0)) sdummy = (const char *) string_temp; ; ; ; pdummy = (unsigned char *) fail_stack.stack[--fail_stack.avail].pointer; ; ; dummy_high_reg = (active_reg_t) fail_stack.stack[--fail_stack.avail].integer; ; dummy_low_reg = (active_reg_t) fail_stack.stack[--fail_stack.avail].integer; ; if (1) for (this_reg = dummy_high_reg; this_reg >= dummy_low_reg; this_reg--) { ; reg_info_dummy[this_reg].word = fail_stack.stack[--fail_stack.avail]; ; reg_dummy[this_reg] = (const char *) fail_stack.stack[--fail_stack.avail].pointer; ; reg_dummy[this_reg] = (const char *) fail_stack.stack[--fail_stack.avail].pointer; ; } else { for (this_reg = highest_active_reg; this_reg > dummy_high_reg; this_reg--) { reg_info_dummy[this_reg].word.integer = 0; reg_dummy[this_reg] = 0; reg_dummy[this_reg] = 0; } highest_active_reg = dummy_high_reg; } set_regs_matched_done = 0; ; }
                                                                    ;
          }
 unconditional_jump:
   ;
        case jump:
   do { do { (mcnt) = *(p) & 0377; (mcnt) += ((signed char) (*((p) + 1))) << 8; } while (0); (p) += 2; } while (0);
          ;
   p += mcnt;
          ;
   break;
        case jump_past_alt:
          ;
          goto unconditional_jump;
        case dummy_failure_jump:
          ;
          do { char *destination; active_reg_t this_reg; ; ; ; ; ; ; ; while (((fail_stack).size - (fail_stack).avail) < (((0 ? 0 : highest_active_reg - lowest_active_reg + 1) * 3) + 4)) { if (!((fail_stack).size > (unsigned) (xre_max_failures * (5 * 3 + 4)) ? 0 : ((fail_stack).stack = (byte_fail_stack_elt_t *) (destination = (char *) alloca (((fail_stack).size << 1) * sizeof (byte_fail_stack_elt_t)), memcpy (destination, (fail_stack).stack, (fail_stack).size * sizeof (byte_fail_stack_elt_t))), (fail_stack).stack == ((void *)0) ? 0 : ((fail_stack).size <<= 1, 1)))) return -2; ; ; } ; if (1) for (this_reg = lowest_active_reg; this_reg <= highest_active_reg; this_reg++) { ; ; ; fail_stack.stack[fail_stack.avail++].pointer = (unsigned char *) (regstart[this_reg]); ; fail_stack.stack[fail_stack.avail++].pointer = (unsigned char *) (regend[this_reg]); ; ; ; ; ; ; fail_stack.stack[fail_stack.avail++] = (reg_info[this_reg].word); } ; fail_stack.stack[fail_stack.avail++].integer = (lowest_active_reg); ; fail_stack.stack[fail_stack.avail++].integer = (highest_active_reg); ; ; fail_stack.stack[fail_stack.avail++].pointer = (unsigned char *) (((void *)0)); ; ; ; fail_stack.stack[fail_stack.avail++].pointer = (unsigned char *) (((void *)0)); ; ; } while (0);
          goto unconditional_jump;
        case push_dummy_failure:
          ;
          do { char *destination; active_reg_t this_reg; ; ; ; ; ; ; ; while (((fail_stack).size - (fail_stack).avail) < (((0 ? 0 : highest_active_reg - lowest_active_reg + 1) * 3) + 4)) { if (!((fail_stack).size > (unsigned) (xre_max_failures * (5 * 3 + 4)) ? 0 : ((fail_stack).stack = (byte_fail_stack_elt_t *) (destination = (char *) alloca (((fail_stack).size << 1) * sizeof (byte_fail_stack_elt_t)), memcpy (destination, (fail_stack).stack, (fail_stack).size * sizeof (byte_fail_stack_elt_t))), (fail_stack).stack == ((void *)0) ? 0 : ((fail_stack).size <<= 1, 1)))) return -2; ; ; } ; if (1) for (this_reg = lowest_active_reg; this_reg <= highest_active_reg; this_reg++) { ; ; ; fail_stack.stack[fail_stack.avail++].pointer = (unsigned char *) (regstart[this_reg]); ; fail_stack.stack[fail_stack.avail++].pointer = (unsigned char *) (regend[this_reg]); ; ; ; ; ; ; fail_stack.stack[fail_stack.avail++] = (reg_info[this_reg].word); } ; fail_stack.stack[fail_stack.avail++].integer = (lowest_active_reg); ; fail_stack.stack[fail_stack.avail++].integer = (highest_active_reg); ; ; fail_stack.stack[fail_stack.avail++].pointer = (unsigned char *) (((void *)0)); ; ; ; fail_stack.stack[fail_stack.avail++].pointer = (unsigned char *) (((void *)0)); ; ; } while (0);
          break;
        case succeed_n:
          do { (mcnt) = *(p + 2) & 0377; (mcnt) += ((signed char) (*((p + 2) + 1))) << 8; } while (0);
          ;
          ;
          if (mcnt > 0)
            {
               mcnt--;
        p += 2;
               do { do { (p)[0] = (mcnt) & 0377; (p)[1] = (mcnt) >> 8; } while (0); (p) += 2; } while (0);
              
               ;
            }
   else if (mcnt == 0)
            {
             
                               ;
       p[2] = (unsigned char) no_op;
              p[3] = (unsigned char) no_op;
              goto on_failure;
            }
          break;
        case jump_n:
          do { (mcnt) = *(p + 2) & 0377; (mcnt) += ((signed char) (*((p + 2) + 1))) << 8; } while (0);
          ;
          if (mcnt)
            {
               mcnt--;
               do { (p + 2)[0] = (mcnt) & 0377; (p + 2)[1] = (mcnt) >> 8; } while (0);
              
             ;
        goto unconditional_jump;
            }
   else
     p += 2 * 2;
          break;
 case set_number_at:
   {
            ;
            do { do { (mcnt) = *(p) & 0377; (mcnt) += ((signed char) (*((p) + 1))) << 8; } while (0); (p) += 2; } while (0);
            p1 = p + mcnt;
            do { do { (mcnt) = *(p) & 0377; (mcnt) += ((signed char) (*((p) + 1))) << 8; } while (0); (p) += 2; } while (0);
            ;
     do { (p1)[0] = (mcnt) & 0377; (p1)[1] = (mcnt) >> 8; } while (0);
            break;
          }
 case wordbound:
 {
   boolean prevchar, thischar;
   ;
   if (((d) == (size1 ? string1 : string2) || !size2) || ((d) == end2))
     break;
   prevchar = (re_syntax_table[(unsigned char) ((d - 1) == end1 ? *string2 : (d - 1) == string2 - 1 ? *(end1 - 1) : *(d - 1))] == 1);
   thischar = (re_syntax_table[(unsigned char) ((d) == end1 ? *string2 : (d) == string2 - 1 ? *(end1 - 1) : *(d))] == 1);
   if (prevchar != thischar)
     break;
   goto fail;
 }
      case notwordbound:
 {
   boolean prevchar, thischar;
   ;
   if (((d) == (size1 ? string1 : string2) || !size2) || ((d) == end2))
     goto fail;
   prevchar = (re_syntax_table[(unsigned char) ((d - 1) == end1 ? *string2 : (d - 1) == string2 - 1 ? *(end1 - 1) : *(d - 1))] == 1);
   thischar = (re_syntax_table[(unsigned char) ((d) == end1 ? *string2 : (d) == string2 - 1 ? *(end1 - 1) : *(d))] == 1);
   if (prevchar != thischar)
     goto fail;
   break;
 }
 case wordbeg:
          ;
   if (!((d) == end2) && (re_syntax_table[(unsigned char) ((d) == end1 ? *string2 : (d) == string2 - 1 ? *(end1 - 1) : *(d))] == 1)
       && (((d) == (size1 ? string1 : string2) || !size2) || !(re_syntax_table[(unsigned char) ((d - 1) == end1 ? *string2 : (d - 1) == string2 - 1 ? *(end1 - 1) : *(d - 1))] == 1)))
     break;
          goto fail;
 case wordend:
          ;
   if (!((d) == (size1 ? string1 : string2) || !size2) && (re_syntax_table[(unsigned char) ((d - 1) == end1 ? *string2 : (d - 1) == string2 - 1 ? *(end1 - 1) : *(d - 1))] == 1)
              && (((d) == end2) || !(re_syntax_table[(unsigned char) ((d) == end1 ? *string2 : (d) == string2 - 1 ? *(end1 - 1) : *(d))] == 1)))
     break;
          goto fail;
 case wordchar:
          ;
   while (d == dend) { if (dend == end_match_2) goto fail; d = string2; dend = end_match_2; };
          if (!(re_syntax_table[(unsigned char) ((d) == end1 ? *string2 : (d) == string2 - 1 ? *(end1 - 1) : *(d))] == 1))
            goto fail;
   do { if (!set_regs_matched_done) { active_reg_t r; set_regs_matched_done = 1; for (r = lowest_active_reg; r <= highest_active_reg; r++) { ((reg_info[r]).bits.matched_something) = ((reg_info[r]).bits.ever_matched_something) = 1; } } } while (0);
          d++;
   break;
 case notwordchar:
          ;
   while (d == dend) { if (dend == end_match_2) goto fail; d = string2; dend = end_match_2; };
   if ((re_syntax_table[(unsigned char) ((d) == end1 ? *string2 : (d) == string2 - 1 ? *(end1 - 1) : *(d))] == 1))
            goto fail;
          do { if (!set_regs_matched_done) { active_reg_t r; set_regs_matched_done = 1; for (r = lowest_active_reg; r <= highest_active_reg; r++) { ((reg_info[r]).bits.matched_something) = ((reg_info[r]).bits.ever_matched_something) = 1; } } } while (0);
          d++;
   break;
        default:
          abort ();
 }
      continue;
    fail:
      if (!(fail_stack.avail == 0))
 {
          ;
          { active_reg_t this_reg; const unsigned char *string_temp; ; ; ; ; ; ; ; string_temp = fail_stack.stack[--fail_stack.avail].pointer; if (string_temp != ((void *)0)) d = (const char *) string_temp; ; ; ; p = (unsigned char *) fail_stack.stack[--fail_stack.avail].pointer; ; ; highest_active_reg = (active_reg_t) fail_stack.stack[--fail_stack.avail].integer; ; lowest_active_reg = (active_reg_t) fail_stack.stack[--fail_stack.avail].integer; ; if (1) for (this_reg = highest_active_reg; this_reg >= lowest_active_reg; this_reg--) { ; reg_info[this_reg].word = fail_stack.stack[--fail_stack.avail]; ; regend[this_reg] = (const char *) fail_stack.stack[--fail_stack.avail].pointer; ; regstart[this_reg] = (const char *) fail_stack.stack[--fail_stack.avail].pointer; ; } else { for (this_reg = highest_active_reg; this_reg > highest_active_reg; this_reg--) { reg_info[this_reg].word.integer = 0; regend[this_reg] = 0; regstart[this_reg] = 0; } highest_active_reg = highest_active_reg; } set_regs_matched_done = 0; ; }
                                                        ;
          if (!p)
     goto fail;
   ;
          if (p < pend)
            {
              boolean is_a_jump_n = 0;
              switch ((re_opcode_t) *p)
                {
                case jump_n:
                  is_a_jump_n = 1;
                case maybe_pop_jump:
                case pop_failure_jump:
                case jump:
                  p1 = p + 1;
                  do { do { (mcnt) = *(p1) & 0377; (mcnt) += ((signed char) (*((p1) + 1))) << 8; } while (0); (p1) += 2; } while (0);
                  p1 += mcnt;
                  if ((is_a_jump_n && (re_opcode_t) *p1 == succeed_n)
                      || (!is_a_jump_n
                          && (re_opcode_t) *p1 == on_failure_jump))
                    goto fail;
                  break;
                default:
                                   ;
                }
            }
          if (d >= string1 && d <= end1)
     dend = end_match_1;
        }
      else
        break;
    }
  if (best_regs_set)
    goto restore_best_regs;
  do { ; if (regstart) ((void)0); regstart = ((void *)0); if (regend) ((void)0); regend = ((void *)0); if (old_regstart) ((void)0); old_regstart = ((void *)0); if (old_regend) ((void)0); old_regend = ((void *)0); if (best_regstart) ((void)0); best_regstart = ((void *)0); if (best_regend) ((void)0); best_regend = ((void *)0); if (reg_info) ((void)0); reg_info = ((void *)0); if (reg_dummy) ((void)0); reg_dummy = ((void *)0); if (reg_info_dummy) ((void)0); reg_info_dummy = ((void *)0); } while (0);
  return -1;
}
static boolean
byte_group_match_null_string_p (p, end, reg_info)
    unsigned char *p, *end;
    byte_register_info_type *reg_info;
{
  int mcnt;
  unsigned char *p1 = *p + 2;
  while (p1 < end)
    {
      switch ((re_opcode_t) *p1)
        {
        case on_failure_jump:
          p1++;
          do { do { (mcnt) = *(p1) & 0377; (mcnt) += ((signed char) (*((p1) + 1))) << 8; } while (0); (p1) += 2; } while (0);
   if (mcnt >= 0)
     {
              while ((re_opcode_t) p1[mcnt-(1+2)] ==
       jump_past_alt)
                {
                  if (!byte_alt_match_null_string_p (p1, p1 + mcnt -
      (1 + 2),
      reg_info))
                    return 0;
                  p1 += mcnt;
                  if ((re_opcode_t) *p1 != on_failure_jump)
                    break;
    p1++;
                  do { do { (mcnt) = *(p1) & 0377; (mcnt) += ((signed char) (*((p1) + 1))) << 8; } while (0); (p1) += 2; } while (0);
                  if ((re_opcode_t) p1[mcnt-(1+2)] !=
        jump_past_alt)
                    {
                      p1 -= 1 + 2;
                      break;
                    }
                }
              do { (mcnt) = *(p1 - 2) & 0377; (mcnt) += ((signed char) (*((p1 - 2) + 1))) << 8; } while (0);
              if (!byte_alt_match_null_string_p (p1, p1 + mcnt, reg_info))
                return 0;
              p1 += mcnt;
            }
          break;
        case stop_memory:
   ;
          *p = p1 + 2;
          return 1;
        default:
          if (!byte_common_op_match_null_string_p (&p1, end, reg_info))
            return 0;
        }
    }
  return 0;
}
static boolean
byte_alt_match_null_string_p (p, end, reg_info)
    unsigned char *p, *end;
    byte_register_info_type *reg_info;
{
  int mcnt;
  unsigned char *p1 = p;
  while (p1 < end)
    {
      switch ((re_opcode_t) *p1)
        {
        case on_failure_jump:
          p1++;
          do { do { (mcnt) = *(p1) & 0377; (mcnt) += ((signed char) (*((p1) + 1))) << 8; } while (0); (p1) += 2; } while (0);
          p1 += mcnt;
          break;
 default:
          if (!byte_common_op_match_null_string_p (&p1, end, reg_info))
            return 0;
        }
    }
  return 1;
}
static boolean
byte_common_op_match_null_string_p (p, end, reg_info)
    unsigned char *p, *end;
    byte_register_info_type *reg_info;
{
  int mcnt;
  boolean ret;
  int reg_no;
  unsigned char *p1 = *p;
  switch ((re_opcode_t) *p1++)
    {
    case no_op:
    case begline:
    case endline:
    case begbuf:
    case endbuf:
    case wordbeg:
    case wordend:
    case wordbound:
    case notwordbound:
      break;
    case start_memory:
      reg_no = *p1;
      ;
      ret = byte_group_match_null_string_p (&p1, end, reg_info);
      if (((reg_info[reg_no]).bits.match_null_string_p) == 3)
        ((reg_info[reg_no]).bits.match_null_string_p) = ret;
      if (!ret)
        return 0;
      break;
    case jump:
      do { do { (mcnt) = *(p1) & 0377; (mcnt) += ((signed char) (*((p1) + 1))) << 8; } while (0); (p1) += 2; } while (0);
      if (mcnt >= 0)
        p1 += mcnt;
      else
        return 0;
      break;
    case succeed_n:
      p1 += 2;
      do { do { (mcnt) = *(p1) & 0377; (mcnt) += ((signed char) (*((p1) + 1))) << 8; } while (0); (p1) += 2; } while (0);
      if (mcnt == 0)
        {
          p1 -= 2 * 2;
          do { do { (mcnt) = *(p1) & 0377; (mcnt) += ((signed char) (*((p1) + 1))) << 8; } while (0); (p1) += 2; } while (0);
          p1 += mcnt;
        }
      else
        return 0;
      break;
    case duplicate:
      if (!((reg_info[*p1]).bits.match_null_string_p))
        return 0;
      break;
    case set_number_at:
      p1 += 2 * 2;
    default:
      return 0;
  }
  *p = p1;
  return 1;
}
static int
byte_bcmp_translate (s1, s2, len, translate)
     const char *s1, *s2;
     register int len;
     char * translate;
{
  register const unsigned char *p1 = (const unsigned char *) s1;
  register const unsigned char *p2 = (const unsigned char *) s2;
  while (len)
    {
      if (translate[*p1++] != translate[*p2++]) return 1;
      len--;
    }
  return 0;
}
reg_syntax_t xre_syntax_options;
reg_syntax_t
xre_set_syntax (syntax)
    reg_syntax_t syntax;
{
  reg_syntax_t ret = xre_syntax_options;
  xre_syntax_options = syntax;
  return ret;
}
static const char *re_error_msgid[] =
  {
    "Success",
    "No match",
    "Invalid regular expression",
    "Invalid collation character",
    "Invalid character class name",
    "Trailing backslash",
    "Invalid back reference",
    "Unmatched [ or [^",
    "Unmatched ( or \\(",
    "Unmatched \\{",
    "Invalid content of \\{\\}",
    "Invalid range end",
    "Memory exhausted",
    "Invalid preceding regular expression",
    "Premature end of regular expression",
    "Regular expression too big",
    "Unmatched ) or \\)"
  };
static boolean
group_in_compile_stack (compile_stack, regnum)
    compile_stack_type compile_stack;
    regnum_t regnum;
{
  int this_element;
  for (this_element = compile_stack.avail - 1;
       this_element >= 0;
       this_element--)
    if (compile_stack.stack[this_element].regnum == regnum)
      return 1;
  return 0;
}
int
xre_compile_fastmap (bufp)
     struct re_pattern_buffer *bufp;
{
    return byte_re_compile_fastmap(bufp);
}
void
xre_set_registers (bufp, regs, num_regs, starts, ends)
    struct re_pattern_buffer *bufp;
    struct re_registers *regs;
    unsigned num_regs;
    regoff_t *starts, *ends;
{
  if (num_regs)
    {
      bufp->regs_allocated = 1;
      regs->num_regs = num_regs;
      regs->start = starts;
      regs->end = ends;
    }
  else
    {
      bufp->regs_allocated = 0;
      regs->num_regs = 0;
      regs->start = regs->end = (regoff_t *) 0;
    }
}
int
xre_search (bufp, string, size, startpos, range, regs)
     struct re_pattern_buffer *bufp;
     const char *string;
     int size, startpos, range;
     struct re_registers *regs;
{
  return xre_search_2 (bufp, ((void *)0), 0, string, size, startpos, range,
        regs, size);
}
int
xre_search_2 (bufp, string1, size1, string2, size2, startpos, range, regs, stop)
     struct re_pattern_buffer *bufp;
     const char *string1, *string2;
     int size1, size2;
     int startpos;
     int range;
     struct re_registers *regs;
     int stop;
{
    return byte_re_search_2 (bufp, string1, size1, string2, size2, startpos,
        range, regs, stop);
}
int
xre_match (bufp, string, size, pos, regs)
     struct re_pattern_buffer *bufp;
     const char *string;
     int size, pos;
     struct re_registers *regs;
{
  int result;
    result = byte_re_match_2_internal (bufp, ((void *)0), 0, string, size,
      pos, regs, size);
  return result;
}
int
xre_match_2 (bufp, string1, size1, string2, size2, pos, regs, stop)
     struct re_pattern_buffer *bufp;
     const char *string1, *string2;
     int size1, size2;
     int pos;
     struct re_registers *regs;
     int stop;
{
  int result;
    result = byte_re_match_2_internal (bufp, string1, size1, string2, size2,
      pos, regs, stop);
  return result;
}
const char *
xre_compile_pattern (pattern, length, bufp)
     const char *pattern;
     size_t length;
     struct re_pattern_buffer *bufp;
{
  reg_errcode_t ret;
  bufp->regs_allocated = 0;
  bufp->no_sub = 0;
  bufp->newline_anchor = 1;
    ret = byte_regex_compile (pattern, length, xre_syntax_options, bufp);
  if (!ret)
    return ((void *)0);
  return gettext (re_error_msgid[(int) ret]);
}
static struct re_pattern_buffer re_comp_buf;
char *
xre_comp (s)
    const char *s;
{
  reg_errcode_t ret;
  if (!s)
    {
      if (!re_comp_buf.buffer)
 return gettext ("No previous regular expression");
      return 0;
    }
  if (!re_comp_buf.buffer)
    {
      re_comp_buf.buffer = (unsigned char *) malloc (200);
      if (re_comp_buf.buffer == ((void *)0))
        return (char *) gettext (re_error_msgid[(int) REG_ESPACE]);
      re_comp_buf.allocated = 200;
      re_comp_buf.fastmap = (char *) malloc (1 << 8);
      if (re_comp_buf.fastmap == ((void *)0))
 return (char *) gettext (re_error_msgid[(int) REG_ESPACE]);
    }
  re_comp_buf.newline_anchor = 1;
    ret = byte_regex_compile (s, strlen (s), xre_syntax_options, &re_comp_buf);
  if (!ret)
    return ((void *)0);
  return (char *) gettext (re_error_msgid[(int) ret]);
}
int
xre_exec (s)
    const char *s;
{
  const int len = strlen (s);
  return
    0 <= xre_search (&re_comp_buf, s, len, 0, len, (struct re_registers *) 0);
}
int
xregcomp (preg, pattern, cflags)
    regex_t *preg;
    const char *pattern;
    int cflags;
{
  reg_errcode_t ret;
  reg_syntax_t syntax
    = (cflags & 1) ?
      ((((((unsigned long int) 1) << 1) << 1) | ((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) | (((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) | (((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) | ((((((((((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1)) | (((((unsigned long int) 1) << 1) << 1) << 1) | ((((((unsigned long int) 1) << 1) << 1) << 1) << 1) | ((((((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) | (((((((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) | (((((((((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) | (((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) | (((((((((((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1)) : ((((((unsigned long int) 1) << 1) << 1) | ((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) | (((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) | (((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) | ((((((((((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1)) | (((unsigned long int) 1) << 1));
  preg->buffer = 0;
  preg->allocated = 0;
  preg->used = 0;
  preg->fastmap = (char *) malloc (1 << 8);
  if (cflags & (1 << 1))
    {
      unsigned i;
      preg->translate
 = (char *) malloc (256
          * sizeof (*(char *)0));
      if (preg->translate == ((void *)0))
        return (int) REG_ESPACE;
      for (i = 0; i < 256; i++)
        preg->translate[i] = (1 && ((*__ctype_b_loc ())[(int) ((i))] & (unsigned short int) _ISupper)) ? ((int) (*__ctype_tolower_loc ())[(int) (i)]) : (int) i;
    }
  else
    preg->translate = ((void *)0);
  if (cflags & ((1 << 1) << 1))
    {
      syntax &= ~((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1);
      syntax |= ((((((((((unsigned long int) 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1);
      preg->newline_anchor = 1;
    }
  else
    preg->newline_anchor = 0;
  preg->no_sub = !!(cflags & (((1 << 1) << 1) << 1));
    ret = byte_regex_compile (pattern, strlen (pattern), syntax, preg);
  if (ret == REG_ERPAREN) ret = REG_EPAREN;
  if (ret == REG_NOERROR && preg->fastmap)
    {
      if (xre_compile_fastmap (preg) == -2)
 {
   free (preg->fastmap);
   preg->fastmap = ((void *)0);
 }
    }
  return (int) ret;
}
int
xregexec (preg, string, nmatch, pmatch, eflags)
    const regex_t *preg;
    const char *string;
    size_t nmatch;
    regmatch_t pmatch[];
    int eflags;
{
  int ret;
  struct re_registers regs;
  regex_t private_preg;
  int len = strlen (string);
  boolean want_reg_info = !preg->no_sub && nmatch > 0;
  private_preg = *preg;
  private_preg.not_bol = !!(eflags & 1);
  private_preg.not_eol = !!(eflags & (1 << 1));
  private_preg.regs_allocated = 2;
  if (want_reg_info)
    {
      regs.num_regs = nmatch;
      regs.start = ((regoff_t *) malloc ((nmatch * 2) * sizeof (regoff_t)));
      if (regs.start == ((void *)0))
        return (int) REG_NOMATCH;
      regs.end = regs.start + nmatch;
    }
  ret = xre_search (&private_preg, string, len,
                                0, len,
                   want_reg_info ? &regs : (struct re_registers *) 0);
  if (want_reg_info)
    {
      if (ret >= 0)
        {
          unsigned r;
          for (r = 0; r < nmatch; r++)
            {
              pmatch[r].rm_so = regs.start[r];
              pmatch[r].rm_eo = regs.end[r];
            }
        }
      free (regs.start);
    }
  return ret >= 0 ? (int) REG_NOERROR : (int) REG_NOMATCH;
}
size_t
xregerror (errcode, preg, errbuf, errbuf_size)
    int errcode;
    const regex_t *preg ;
    char *errbuf;
    size_t errbuf_size;
{
  const char *msg;
  size_t msg_size;
  if (errcode < 0
      || errcode >= (int) (sizeof (re_error_msgid)
      / sizeof (re_error_msgid[0])))
    abort ();
  msg = gettext (re_error_msgid[errcode]);
  msg_size = strlen (msg) + 1;
  if (errbuf_size != 0)
    {
      if (msg_size > errbuf_size)
        {
   *((char *) mempcpy (errbuf, msg, errbuf_size - 1)) = 0;
        }
      else
        memcpy (errbuf, msg, msg_size);
    }
  return msg_size;
}
void
xregfree (preg)
    regex_t *preg;
{
  if (preg->buffer != ((void *)0))
    free (preg->buffer);
  preg->buffer = ((void *)0);
  preg->allocated = 0;
  preg->used = 0;
  if (preg->fastmap != ((void *)0))
    free (preg->fastmap);
  preg->fastmap = ((void *)0);
  preg->fastmap_accurate = 0;
  if (preg->translate != ((void *)0))
    free (preg->translate);
  preg->translate = ((void *)0);
}
enum {
  _sch_isblank = 0x0001,
  _sch_iscntrl = 0x0002,
  _sch_isdigit = 0x0004,
  _sch_islower = 0x0008,
  _sch_isprint = 0x0010,
  _sch_ispunct = 0x0020,
  _sch_isspace = 0x0040,
  _sch_isupper = 0x0080,
  _sch_isxdigit = 0x0100,
  _sch_isidst = 0x0200,
  _sch_isvsp = 0x0400,
  _sch_isnvsp = 0x0800,
  _sch_isalpha = _sch_isupper|_sch_islower,
  _sch_isalnum = _sch_isalpha|_sch_isdigit,
  _sch_isidnum = _sch_isidst|_sch_isdigit,
  _sch_isgraph = _sch_isalnum|_sch_ispunct,
  _sch_iscppsp = _sch_isvsp|_sch_isnvsp,
  _sch_isbasic = _sch_isprint|_sch_iscppsp
};
extern const unsigned short _sch_istable[256];
extern const unsigned char _sch_toupper[256];
extern const unsigned char _sch_tolower[256];
struct _IO_FILE;
typedef struct _IO_FILE FILE;
typedef struct _IO_FILE __FILE;
typedef struct
{
  int __count;
  union
  {
    unsigned int __wch;
    char __wchb[4];
  } __value;
} __mbstate_t;
typedef struct
{
  __off_t __pos;
  __mbstate_t __state;
} _G_fpos_t;
typedef struct
{
  __off64_t __pos;
  __mbstate_t __state;
} _G_fpos64_t;
typedef __builtin_va_list __gnuc_va_list;
struct _IO_jump_t; struct _IO_FILE;
typedef void _IO_lock_t;
struct _IO_marker {
  struct _IO_marker *_next;
  struct _IO_FILE *_sbuf;
  int _pos;
};
enum __codecvt_result
{
  __codecvt_ok,
  __codecvt_partial,
  __codecvt_error,
  __codecvt_noconv
};
struct _IO_FILE {
  int _flags;
  char* _IO_read_ptr;
  char* _IO_read_end;
  char* _IO_read_base;
  char* _IO_write_base;
  char* _IO_write_ptr;
  char* _IO_write_end;
  char* _IO_buf_base;
  char* _IO_buf_end;
  char *_IO_save_base;
  char *_IO_backup_base;
  char *_IO_save_end;
  struct _IO_marker *_markers;
  struct _IO_FILE *_chain;
  int _fileno;
  int _flags2;
  __off_t _old_offset;
  unsigned short _cur_column;
  signed char _vtable_offset;
  char _shortbuf[1];
  _IO_lock_t *_lock;
  __off64_t _offset;
  void *__pad1;
  void *__pad2;
  void *__pad3;
  void *__pad4;
  size_t __pad5;
  int _mode;
  char _unused2[15 * sizeof (int) - 4 * sizeof (void *) - sizeof (size_t)];
};
typedef struct _IO_FILE _IO_FILE;
struct _IO_FILE_plus;
extern struct _IO_FILE_plus _IO_2_1_stdin_;
extern struct _IO_FILE_plus _IO_2_1_stdout_;
extern struct _IO_FILE_plus _IO_2_1_stderr_;
typedef __ssize_t __io_read_fn (void *__cookie, char *__buf, size_t __nbytes);
typedef __ssize_t __io_write_fn (void *__cookie, const char *__buf,
     size_t __n);
typedef int __io_seek_fn (void *__cookie, __off64_t *__pos, int __w);
typedef int __io_close_fn (void *__cookie);
typedef __io_read_fn cookie_read_function_t;
typedef __io_write_fn cookie_write_function_t;
typedef __io_seek_fn cookie_seek_function_t;
typedef __io_close_fn cookie_close_function_t;
typedef struct
{
  __io_read_fn *read;
  __io_write_fn *write;
  __io_seek_fn *seek;
  __io_close_fn *close;
} _IO_cookie_io_functions_t;
typedef _IO_cookie_io_functions_t cookie_io_functions_t;
struct _IO_cookie_file;
extern void _IO_cookie_init (struct _IO_cookie_file *__cfile, int __read_write,
        void *__cookie, _IO_cookie_io_functions_t __fns);
extern int __underflow (_IO_FILE *);
extern int __uflow (_IO_FILE *);
extern int __overflow (_IO_FILE *, int);
extern int _IO_getc (_IO_FILE *__fp);
extern int _IO_putc (int __c, _IO_FILE *__fp);
extern int _IO_feof (_IO_FILE *__fp) ;
extern int _IO_ferror (_IO_FILE *__fp) ;
extern int _IO_peekc_locked (_IO_FILE *__fp);
extern void _IO_flockfile (_IO_FILE *) ;
extern void _IO_funlockfile (_IO_FILE *) ;
extern int _IO_ftrylockfile (_IO_FILE *) ;
extern int _IO_vfscanf (_IO_FILE * , const char * ,
   __gnuc_va_list, int *);
extern int _IO_vfprintf (_IO_FILE *, const char *,
    __gnuc_va_list);
extern __ssize_t _IO_padn (_IO_FILE *, int, __ssize_t);
extern size_t _IO_sgetn (_IO_FILE *, void *, size_t);
extern __off64_t _IO_seekoff (_IO_FILE *, __off64_t, int, int);
extern __off64_t _IO_seekpos (_IO_FILE *, __off64_t, int);
extern void _IO_free_backup_area (_IO_FILE *) ;
typedef __gnuc_va_list va_list;
typedef _G_fpos_t fpos_t;
typedef _G_fpos64_t fpos64_t;
extern struct _IO_FILE *stdin;
extern struct _IO_FILE *stdout;
extern struct _IO_FILE *stderr;
extern int remove (const char *__filename) ;
extern int rename (const char *__old, const char *__new) ;
extern int renameat (int __oldfd, const char *__old, int __newfd,
       const char *__new) ;
extern FILE *tmpfile (void) ;
extern FILE *tmpfile64 (void) ;
extern char *tmpnam (char *__s) ;
extern char *tmpnam_r (char *__s) ;
extern char *tempnam (const char *__dir, const char *__pfx)
     ;
extern int fclose (FILE *__stream);
extern int fflush (FILE *__stream);
extern int fflush_unlocked (FILE *__stream);
extern int fcloseall (void);
extern FILE *fopen (const char * __filename,
      const char * __modes) ;
extern FILE *freopen (const char * __filename,
        const char * __modes,
        FILE * __stream) ;
extern FILE *fopen64 (const char * __filename,
        const char * __modes) ;
extern FILE *freopen64 (const char * __filename,
   const char * __modes,
   FILE * __stream) ;
extern FILE *fdopen (int __fd, const char *__modes) ;
extern FILE *fopencookie (void * __magic_cookie,
     const char * __modes,
     _IO_cookie_io_functions_t __io_funcs) ;
extern FILE *fmemopen (void *__s, size_t __len, const char *__modes)
  ;
extern FILE *open_memstream (char *__bufloc, size_t *__sizeloc) ;
extern void setbuf (FILE * __stream, char * __buf) ;
extern int setvbuf (FILE * __stream, char * __buf,
      int __modes, size_t __n) ;
extern void setbuffer (FILE * __stream, char * __buf,
         size_t __size) ;
extern void setlinebuf (FILE *__stream) ;
extern int fprintf (FILE * __stream,
      const char * __format, ...);
extern int printf (const char * __format, ...);
extern int sprintf (char * __s,
      const char * __format, ...) ;
extern int vfprintf (FILE * __s, const char * __format,
       __gnuc_va_list __arg);
extern int vprintf (const char * __format, __gnuc_va_list __arg);
extern int vsprintf (char * __s, const char * __format,
       __gnuc_va_list __arg) ;
extern int snprintf (char * __s, size_t __maxlen,
       const char * __format, ...)
     ;
extern int vsnprintf (char * __s, size_t __maxlen,
        const char * __format, __gnuc_va_list __arg)
     ;
extern int vasprintf (char * __ptr, const char * __f,
        __gnuc_va_list __arg)
     ;
extern int __asprintf (char * __ptr,
         const char * __fmt, ...)
     ;
extern int asprintf (char * __ptr,
       const char * __fmt, ...)
     ;
extern int vdprintf (int __fd, const char * __fmt,
       __gnuc_va_list __arg)
     ;
extern int dprintf (int __fd, const char * __fmt, ...)
     ;
extern int fscanf (FILE * __stream,
     const char * __format, ...) ;
extern int scanf (const char * __format, ...) ;
extern int sscanf (const char * __s,
     const char * __format, ...) ;
extern int vfscanf (FILE * __s, const char * __format,
      __gnuc_va_list __arg)
     ;
extern int vscanf (const char * __format, __gnuc_va_list __arg)
     ;
extern int vsscanf (const char * __s,
      const char * __format, __gnuc_va_list __arg)
     ;
extern int fgetc (FILE *__stream);
extern int getc (FILE *__stream);
extern int getchar (void);
extern int getc_unlocked (FILE *__stream);
extern int getchar_unlocked (void);
extern int fgetc_unlocked (FILE *__stream);
extern int fputc (int __c, FILE *__stream);
extern int putc (int __c, FILE *__stream);
extern int putchar (int __c);
extern int fputc_unlocked (int __c, FILE *__stream);
extern int putc_unlocked (int __c, FILE *__stream);
extern int putchar_unlocked (int __c);
extern int getw (FILE *__stream);
extern int putw (int __w, FILE *__stream);
extern char *fgets (char * __s, int __n, FILE * __stream)
     ;
extern char *fgets_unlocked (char * __s, int __n,
        FILE * __stream) ;
extern __ssize_t __getdelim (char * __lineptr,
          size_t * __n, int __delimiter,
          FILE * __stream) ;
extern __ssize_t getdelim (char * __lineptr,
        size_t * __n, int __delimiter,
        FILE * __stream) ;
extern __ssize_t getline (char * __lineptr,
       size_t * __n,
       FILE * __stream) ;
extern int fputs (const char * __s, FILE * __stream);
extern int puts (const char *__s);
extern int ungetc (int __c, FILE *__stream);
extern size_t fread (void * __ptr, size_t __size,
       size_t __n, FILE * __stream) ;
extern size_t fwrite (const void * __ptr, size_t __size,
        size_t __n, FILE * __s);
extern int fputs_unlocked (const char * __s,
      FILE * __stream);
extern size_t fread_unlocked (void * __ptr, size_t __size,
         size_t __n, FILE * __stream) ;
extern size_t fwrite_unlocked (const void * __ptr, size_t __size,
          size_t __n, FILE * __stream);
extern int fseek (FILE *__stream, long int __off, int __whence);
extern long int ftell (FILE *__stream) ;
extern void rewind (FILE *__stream);
extern int fseeko (FILE *__stream, __off_t __off, int __whence);
extern __off_t ftello (FILE *__stream) ;
extern int fgetpos (FILE * __stream, fpos_t * __pos);
extern int fsetpos (FILE *__stream, const fpos_t *__pos);
extern int fseeko64 (FILE *__stream, __off64_t __off, int __whence);
extern __off64_t ftello64 (FILE *__stream) ;
extern int fgetpos64 (FILE * __stream, fpos64_t * __pos);
extern int fsetpos64 (FILE *__stream, const fpos64_t *__pos);
extern void clearerr (FILE *__stream) ;
extern int feof (FILE *__stream) ;
extern int ferror (FILE *__stream) ;
extern void clearerr_unlocked (FILE *__stream) ;
extern int feof_unlocked (FILE *__stream) ;
extern int ferror_unlocked (FILE *__stream) ;
extern void perror (const char *__s);
extern int sys_nerr;
extern const char *const sys_errlist[];
extern int _sys_nerr;
extern const char *const _sys_errlist[];
extern int fileno (FILE *__stream) ;
extern int fileno_unlocked (FILE *__stream) ;
extern FILE *popen (const char *__command, const char *__modes) ;
extern int pclose (FILE *__stream);
extern char *ctermid (char *__s) ;
extern char *cuserid (char *__s);
struct obstack;
extern int obstack_printf (struct obstack * __obstack,
      const char * __format, ...)
     ;
extern int obstack_vprintf (struct obstack * __obstack,
       const char * __format,
       __gnuc_va_list __args)
     ;
