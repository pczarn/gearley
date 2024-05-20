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
extern void flockfile (FILE *__stream) ;
extern int ftrylockfile (FILE *__stream) ;
extern void funlockfile (FILE *__stream) ;
extern char *buildargv (const char *) ;
extern void freeargv (char *);
extern char *dupargv (char *) ;
extern const char *lbasename (const char *);
extern char *lrealpath (const char *);
extern char *concat (const char *, ...) ;
extern char *reconcat (char *, const char *, ...) ;
extern unsigned long concat_length (const char *, ...);
extern char *concat_copy (char *, const char *, ...);
extern char *concat_copy2 (const char *, ...);
extern char *libiberty_concat_ptr;
extern int fdmatch (int fd1, int fd2);
extern char * getpwd (void);
extern long get_run_time (void);
extern char *make_relative_prefix (const char *, const char *, const char *)
                      ;
extern char *choose_temp_base (void) ;
extern char *make_temp_file (const char *) ;
extern const char *spaces (int count);
extern int errno_max (void);
extern const char *strerrno (int);
extern int strtoerrno (const char *);
extern char *xstrerror (int);
extern int signo_max (void);
extern const char *strsigno (int);
extern int strtosigno (const char *);
extern int xatexit (void (*fn) (void));
extern void xexit (int status) ;
extern void xmalloc_set_program_name (const char *);
extern void xmalloc_failed (size_t) ;
extern void * xmalloc (size_t) ;
extern void * xrealloc (void *, size_t);
extern void * xcalloc (size_t, size_t) ;
extern char *xstrdup (const char *) ;
extern void * xmemdup (const void *, size_t, size_t) ;
extern double physmem_total (void);
extern double physmem_available (void);
extern const unsigned char _hex_value[256];
extern void hex_init (void);
extern int pexecute (const char *, char * const *, const char *, const char *, char *, char *, int)
                                            ;
extern int pwait (int, int *, int);
extern int asprintf (char *, const char *, ...) ;
extern void * C_alloca (size_t) ;
extern const char *libiberty_optr;
extern char *libiberty_nptr;
extern unsigned long libiberty_len;
extern enum demangling_styles
{
  no_demangling = -1,
  unknown_demangling = 0,
  auto_demangling = (1 << 8),
  gnu_demangling = (1 << 9),
  lucid_demangling = (1 << 10),
  arm_demangling = (1 << 11),
  hp_demangling = (1 << 12),
  edg_demangling = (1 << 13),
  gnu_v3_demangling = (1 << 14),
  java_demangling = (1 << 2),
  gnat_demangling = (1 << 15)
} current_demangling_style;
extern const struct demangler_engine
{
  const char *const demangling_style_name;
  const enum demangling_styles demangling_style;
  const char *const demangling_style_doc;
} libiberty_demanglers[];
extern char *
cplus_demangle (const char *mangled, int options);
extern int
cplus_demangle_opname (const char *opname, char *result, int options);
extern const char *
cplus_mangle_opname (const char *opname, int options);
extern void
set_cplus_marker_for_demangling (int ch);
extern enum demangling_styles
cplus_demangle_set_style (enum demangling_styles style);
extern enum demangling_styles
cplus_demangle_name_to_style (const char *name);
extern char*
cplus_demangle_v3 (const char* mangled, int options);
extern char*
java_demangle_v3 (const char* mangled);
enum gnu_v3_ctor_kinds {
  gnu_v3_complete_object_ctor = 1,
  gnu_v3_base_object_ctor,
  gnu_v3_complete_object_allocating_ctor
};
extern enum gnu_v3_ctor_kinds
 is_gnu_v3_mangled_ctor (const char *name);
enum gnu_v3_dtor_kinds {
  gnu_v3_deleting_dtor = 1,
  gnu_v3_complete_object_dtor,
  gnu_v3_base_object_dtor
};
extern enum gnu_v3_dtor_kinds
 is_gnu_v3_mangled_dtor (const char *name);
enum demangle_component_type
{
  DEMANGLE_COMPONENT_NAME,
  DEMANGLE_COMPONENT_QUAL_NAME,
  DEMANGLE_COMPONENT_LOCAL_NAME,
  DEMANGLE_COMPONENT_TYPED_NAME,
  DEMANGLE_COMPONENT_TEMPLATE,
  DEMANGLE_COMPONENT_TEMPLATE_PARAM,
  DEMANGLE_COMPONENT_CTOR,
  DEMANGLE_COMPONENT_DTOR,
  DEMANGLE_COMPONENT_VTABLE,
  DEMANGLE_COMPONENT_VTT,
  DEMANGLE_COMPONENT_CONSTRUCTION_VTABLE,
  DEMANGLE_COMPONENT_TYPEINFO,
  DEMANGLE_COMPONENT_TYPEINFO_NAME,
  DEMANGLE_COMPONENT_TYPEINFO_FN,
  DEMANGLE_COMPONENT_THUNK,
  DEMANGLE_COMPONENT_VIRTUAL_THUNK,
  DEMANGLE_COMPONENT_COVARIANT_THUNK,
  DEMANGLE_COMPONENT_JAVA_CLASS,
  DEMANGLE_COMPONENT_GUARD,
  DEMANGLE_COMPONENT_REFTEMP,
  DEMANGLE_COMPONENT_SUB_STD,
  DEMANGLE_COMPONENT_RESTRICT,
  DEMANGLE_COMPONENT_VOLATILE,
  DEMANGLE_COMPONENT_CONST,
  DEMANGLE_COMPONENT_RESTRICT_THIS,
  DEMANGLE_COMPONENT_VOLATILE_THIS,
  DEMANGLE_COMPONENT_CONST_THIS,
  DEMANGLE_COMPONENT_VENDOR_TYPE_QUAL,
  DEMANGLE_COMPONENT_POINTER,
  DEMANGLE_COMPONENT_REFERENCE,
  DEMANGLE_COMPONENT_COMPLEX,
  DEMANGLE_COMPONENT_IMAGINARY,
  DEMANGLE_COMPONENT_BUILTIN_TYPE,
  DEMANGLE_COMPONENT_VENDOR_TYPE,
  DEMANGLE_COMPONENT_FUNCTION_TYPE,
  DEMANGLE_COMPONENT_ARRAY_TYPE,
  DEMANGLE_COMPONENT_PTRMEM_TYPE,
  DEMANGLE_COMPONENT_ARGLIST,
  DEMANGLE_COMPONENT_TEMPLATE_ARGLIST,
  DEMANGLE_COMPONENT_OPERATOR,
  DEMANGLE_COMPONENT_EXTENDED_OPERATOR,
  DEMANGLE_COMPONENT_CAST,
  DEMANGLE_COMPONENT_UNARY,
  DEMANGLE_COMPONENT_BINARY,
  DEMANGLE_COMPONENT_BINARY_ARGS,
  DEMANGLE_COMPONENT_TRINARY,
  DEMANGLE_COMPONENT_TRINARY_ARG1,
  DEMANGLE_COMPONENT_TRINARY_ARG2,
  DEMANGLE_COMPONENT_LITERAL,
  DEMANGLE_COMPONENT_LITERAL_NEG
};
struct demangle_operator_info;
struct demangle_builtin_type_info;
struct demangle_component
{
  enum demangle_component_type type;
  union
  {
    struct
    {
      const char *s;
      int len;
    } s_name;
    struct
    {
      const struct demangle_operator_info *op;
    } s_operator;
    struct
    {
      int args;
      struct demangle_component *name;
    } s_extended_operator;
    struct
    {
      enum gnu_v3_ctor_kinds kind;
      struct demangle_component *name;
    } s_ctor;
    struct
    {
      enum gnu_v3_dtor_kinds kind;
      struct demangle_component *name;
    } s_dtor;
    struct
    {
      const struct demangle_builtin_type_info *type;
    } s_builtin;
    struct
    {
      const char* string;
      int len;
    } s_string;
    struct
    {
      long number;
    } s_number;
    struct
    {
      struct demangle_component *left;
      struct demangle_component *right;
    } s_binary;
  } u;
};
extern int
cplus_demangle_fill_component (struct demangle_component *fill, enum demangle_component_type, struct demangle_component *left, struct demangle_component *right)
                                             ;
extern int
cplus_demangle_fill_name (struct demangle_component *fill, const char *, int)
                         ;
extern int
cplus_demangle_fill_builtin_type (struct demangle_component *fill, const char *type_name)
                              ;
extern int
cplus_demangle_fill_operator (struct demangle_component *fill, const char *opname, int args)
                                        ;
extern int
cplus_demangle_fill_extended_operator (struct demangle_component *fill, int numargs, struct demangle_component *nm)
                                           ;
extern int
cplus_demangle_fill_ctor (struct demangle_component *fill, enum gnu_v3_ctor_kinds kind, struct demangle_component *name)
                                       ;
extern int
cplus_demangle_fill_dtor (struct demangle_component *fill, enum gnu_v3_dtor_kinds kind, struct demangle_component *name)
                                       ;
extern struct demangle_component *
cplus_demangle_v3_components (const char *mangled, int options, void *mem)
                      ;
extern char *
cplus_demangle_print (int options, const struct demangle_component *tree, int estimated_length, size_t *p_allocated_size)
                                   ;
static char *ada_demangle (const char *, int);
enum demangling_styles current_demangling_style = auto_demangling;
static char cplus_markers[] = { '$', '.', '$', 0 };
static char char_str[2] = { 0, 0 };
void
set_cplus_marker_for_demangling (ch)
     int ch;
{
  cplus_markers[0] = ch;
}
typedef struct string
{
  char *b;
  char *p;
  char *e;
} string;
struct work_stuff
{
  int options;
  char *typevec;
  char *ktypevec;
  char *btypevec;
  int numk;
  int numb;
  int ksize;
  int bsize;
  int ntypes;
  int typevec_size;
  int constructor;
  int destructor;
  int static_type;
  int temp_start;
  int type_quals;
  int dllimported;
  char *tmpl_argvec;
  int ntmpl_args;
  int forgetting_types;
  string* previous_argument;
  int nrepeats;
};
static const struct optable
{
  const char *const in;
  const char *const out;
  const int flags;
} optable[] = {
  {"nw", " new", (1 << 1)},
  {"dl", " delete", (1 << 1)},
  {"new", " new", 0},
  {"delete", " delete", 0},
  {"vn", " new []", (1 << 1)},
  {"vd", " delete []", (1 << 1)},
  {"as", "=", (1 << 1)},
  {"ne", "!=", (1 << 1)},
  {"eq", "==", (1 << 1)},
  {"ge", ">=", (1 << 1)},
  {"gt", ">", (1 << 1)},
  {"le", "<=", (1 << 1)},
  {"lt", "<", (1 << 1)},
  {"plus", "+", 0},
  {"pl", "+", (1 << 1)},
  {"apl", "+=", (1 << 1)},
  {"minus", "-", 0},
  {"mi", "-", (1 << 1)},
  {"ami", "-=", (1 << 1)},
  {"mult", "*", 0},
  {"ml", "*", (1 << 1)},
  {"amu", "*=", (1 << 1)},
  {"aml", "*=", (1 << 1)},
  {"convert", "+", 0},
  {"negate", "-", 0},
  {"trunc_mod", "%", 0},
  {"md", "%", (1 << 1)},
  {"amd", "%=", (1 << 1)},
  {"trunc_div", "/", 0},
  {"dv", "/", (1 << 1)},
  {"adv", "/=", (1 << 1)},
  {"truth_andif", "&&", 0},
  {"aa", "&&", (1 << 1)},
  {"truth_orif", "||", 0},
  {"oo", "||", (1 << 1)},
  {"truth_not", "!", 0},
  {"nt", "!", (1 << 1)},
  {"postincrement","++", 0},
  {"pp", "++", (1 << 1)},
  {"postdecrement","--", 0},
  {"mm", "--", (1 << 1)},
  {"bit_ior", "|", 0},
  {"or", "|", (1 << 1)},
  {"aor", "|=", (1 << 1)},
  {"bit_xor", "^", 0},
  {"er", "^", (1 << 1)},
  {"aer", "^=", (1 << 1)},
  {"bit_and", "&", 0},
  {"ad", "&", (1 << 1)},
  {"aad", "&=", (1 << 1)},
  {"bit_not", "~", 0},
  {"co", "~", (1 << 1)},
  {"call", "()", 0},
  {"cl", "()", (1 << 1)},
  {"alshift", "<<", 0},
  {"ls", "<<", (1 << 1)},
  {"als", "<<=", (1 << 1)},
  {"arshift", ">>", 0},
  {"rs", ">>", (1 << 1)},
  {"ars", ">>=", (1 << 1)},
  {"component", "->", 0},
  {"pt", "->", (1 << 1)},
  {"rf", "->", (1 << 1)},
  {"indirect", "*", 0},
  {"method_call", "->()", 0},
  {"addr", "&", 0},
  {"array", "[]", 0},
  {"vc", "[]", (1 << 1)},
  {"compound", ", ", 0},
  {"cm", ", ", (1 << 1)},
  {"cond", "?:", 0},
  {"cn", "?:", (1 << 1)},
  {"max", ">?", 0},
  {"mx", ">?", (1 << 1)},
  {"min", "<?", 0},
  {"mn", "<?", (1 << 1)},
  {"nop", "", 0},
  {"rm", "->*", (1 << 1)},
  {"sz", "sizeof ", (1 << 1)}
};
typedef enum type_kind_t
{
  tk_none,
  tk_pointer,
  tk_reference,
  tk_integral,
  tk_bool,
  tk_char,
  tk_real
} type_kind_t;
const struct demangler_engine libiberty_demanglers[] =
{
  {
    "none",
    no_demangling,
    "Demangling disabled"
  }
  ,
  {
    "auto",
      auto_demangling,
      "Automatic selection based on executable"
  }
  ,
  {
    "gnu",
      gnu_demangling,
      "GNU (g++) style demangling"
  }
  ,
  {
    "lucid",
      lucid_demangling,
      "Lucid (lcc) style demangling"
  }
  ,
  {
    "arm",
      arm_demangling,
      "ARM style demangling"
  }
  ,
  {
    "hp",
      hp_demangling,
      "HP (aCC) style demangling"
  }
  ,
  {
    "edg",
      edg_demangling,
      "EDG style demangling"
  }
  ,
  {
    "gnu-v3",
    gnu_v3_demangling,
    "GNU (g++) V3 ABI-style demangling"
  }
  ,
  {
    "java",
    java_demangling,
    "Java style demangling"
  }
  ,
  {
    "gnat",
    gnat_demangling,
    "GNAT style demangling"
  }
  ,
  {
    ((void *)0), unknown_demangling, ((void *)0)
  }
};
static void
delete_work_stuff (struct work_stuff *);
static void
delete_non_B_K_work_stuff (struct work_stuff *);
static char *
mop_up (struct work_stuff *, string *, int);
static void
squangle_mop_up (struct work_stuff *);
static void
work_stuff_copy_to_from (struct work_stuff *, struct work_stuff *);
static char *
internal_cplus_demangle (struct work_stuff *, const char *);
static int
demangle_template_template_parm (struct work_stuff *work, const char *, string *)
                               ;
static int
demangle_template (struct work_stuff *work, const char *, string *, string *, int, int)
                          ;
static int
arm_pt (struct work_stuff *, const char *, int, const char *, const char *)
                 ;
static int
demangle_class_name (struct work_stuff *, const char *, string *);
static int
demangle_qualified (struct work_stuff *, const char *, string *, int, int)
                 ;
static int
demangle_class (struct work_stuff *, const char *, string *);
static int
demangle_fund_type (struct work_stuff *, const char *, string *);
static int
demangle_signature (struct work_stuff *, const char *, string *);
static int
demangle_prefix (struct work_stuff *, const char *, string *);
static int
gnu_special (struct work_stuff *, const char *, string *);
static int
arm_special (const char *, string *);
static void
string_need (string *, int);
static void
string_delete (string *);
static void
string_init (string *);
static void
string_clear (string *);
static void
string_append (string *, const char *);
static void
string_appends (string *, string *);
static void
string_appendn (string *, const char *, int);
static void
string_prepend (string *, const char *);
static void
string_prependn (string *, const char *, int);
static void
string_append_template_idx (string *, int);
static int
get_count (const char *, int *);
static int
consume_count (const char *);
static int
consume_count_with_underscores (const char*);
static int
demangle_args (struct work_stuff *, const char *, string *);
static int
demangle_nested_args (struct work_stuff*, const char*, string*);
static int
do_type (struct work_stuff *, const char *, string *);
static int
do_arg (struct work_stuff *, const char *, string *);
static void
demangle_function_name (struct work_stuff *, const char *, string *, const char *)
                  ;
static int
iterate_demangle_function (struct work_stuff *, const char *, string *, const char *)
                                              ;
static void
remember_type (struct work_stuff *, const char *, int);
static void
remember_Btype (struct work_stuff *, const char *, int, int);
static int
register_Btype (struct work_stuff *);
static void
remember_Ktype (struct work_stuff *, const char *, int);
static void
forget_types (struct work_stuff *);
static void
forget_B_and_K_types (struct work_stuff *);
static void
string_prepends (string *, string *);
static int
demangle_template_value_parm (struct work_stuff*, const char*, string*, type_kind_t)
                                ;
static int
do_hpacc_template_const_value (struct work_stuff *, const char *, string *);
static int
do_hpacc_template_literal (struct work_stuff *, const char *, string *);
static int
snarf_numeric_literal (const char *, string *);
static int
code_for_qualifier (int);
static const char*
qualifier_string (int);
static const char*
demangle_qualifier (int);
static int
demangle_expression (struct work_stuff *, const char *, string *, type_kind_t)
                     ;
static int
demangle_integral_value (struct work_stuff *, const char *, string *)
               ;
static int
demangle_real_value (struct work_stuff *, const char *, string *);
static void
demangle_arm_hp_template (struct work_stuff *, const char *, int, string *)
                ;
static void
recursively_demangle (struct work_stuff *, const char *, string *, int)
              ;
static void
grow_vect (char *, size_t *, size_t, int);
static int
consume_count (type)
     const char *type;
{
  int count = 0;
  if (! (_sch_istable[((unsigned char)*type) & 0xff] & (unsigned short)(_sch_isdigit)))
    return -1;
  while ((_sch_istable[((unsigned char)*type) & 0xff] & (unsigned short)(_sch_isdigit)))
    {
      count *= 10;
      if ((count % 10) != 0)
 {
   while ((_sch_istable[((unsigned char) *type) & 0xff] & (unsigned short)(_sch_isdigit)))
     (*type)++;
   return -1;
 }
      count += *type - '0';
      (*type)++;
    }
  if (count < 0)
    count = -1;
  return (count);
}
static int
consume_count_with_underscores (mangled)
     const char *mangled;
{
  int idx;
  if (*mangled == '_')
    {
      (*mangled)++;
      if (!(_sch_istable[((unsigned char)*mangled) & 0xff] & (unsigned short)(_sch_isdigit)))
 return -1;
      idx = consume_count (mangled);
      if (*mangled != '_')
 return -1;
      (*mangled)++;
    }
  else
    {
      if (*mangled < '0' || *mangled > '9')
 return -1;
      idx = *mangled - '0';
      (*mangled)++;
    }
  return idx;
}
static int
code_for_qualifier (c)
  int c;
{
  switch (c)
    {
    case 'C':
      return 0x1;
    case 'V':
      return 0x2;
    case 'u':
      return 0x4;
    default:
      break;
    }
  abort ();
}
static const char*
qualifier_string (type_quals)
     int type_quals;
{
  switch (type_quals)
    {
    case 0x0:
      return "";
    case 0x1:
      return "const";
    case 0x2:
      return "volatile";
    case 0x4:
      return "";
    case 0x1 | 0x2:
      return "const volatile";
    case 0x1 | 0x4:
      return "const ";
    case 0x2 | 0x4:
      return "volatile ";
    case 0x1 | 0x2 | 0x4:
      return "const volatile ";
    default:
      break;
    }
  abort ();
}
static const char*
demangle_qualifier (c)
  int c;
{
  return qualifier_string (code_for_qualifier (c));
}
int
cplus_demangle_opname (opname, result, options)
     const char *opname;
     char *result;
     int options;
{
  int len, len1, ret;
  string type;
  struct work_stuff work[1];
  const char *tem;
  len = strlen(opname);
  result[0] = 0;
  ret = 0;
  memset ((char *) work, 0, sizeof (work));
  work->options = options;
  if (opname[0] == '_' && opname[1] == '_'
      && opname[2] == 'o' && opname[3] == 'p')
    {
      tem = opname + 4;
      if (do_type (work, &tem, &type))
 {
   strcat (result, "operator ");
   strncat (result, type.b, type.p - type.b);
   string_delete (&type);
   ret = 1;
 }
    }
  else if (opname[0] == '_' && opname[1] == '_'
    && (_sch_istable[((unsigned char)opname[2]) & 0xff] & (unsigned short)(_sch_islower))
    && (_sch_istable[((unsigned char)opname[3]) & 0xff] & (unsigned short)(_sch_islower)))
    {
      if (opname[4] == 0)
 {
   size_t i;
   for (i = 0; i < (sizeof (optable) / sizeof ((optable)[0])); i++)
     {
       if (strlen (optable[i].in) == 2
    && memcmp (optable[i].in, opname + 2, 2) == 0)
  {
    strcat (result, "operator");
    strcat (result, optable[i].out);
    ret = 1;
    break;
  }
     }
 }
      else
 {
   if (opname[2] == 'a' && opname[5] == 0)
     {
       size_t i;
       for (i = 0; i < (sizeof (optable) / sizeof ((optable)[0])); i++)
  {
    if (strlen (optable[i].in) == 3
        && memcmp (optable[i].in, opname + 2, 3) == 0)
      {
        strcat (result, "operator");
        strcat (result, optable[i].out);
        ret = 1;
        break;
      }
  }
     }
 }
    }
  else if (len >= 3
    && opname[0] == 'o'
    && opname[1] == 'p'
    && strchr (cplus_markers, opname[2]) != ((void *)0))
    {
      if (len >= 10
   && memcmp (opname + 3, "assign_", 7) == 0)
 {
   size_t i;
   for (i = 0; i < (sizeof (optable) / sizeof ((optable)[0])); i++)
     {
       len1 = len - 10;
       if ((int) strlen (optable[i].in) == len1
    && memcmp (optable[i].in, opname + 10, len1) == 0)
  {
    strcat (result, "operator");
    strcat (result, optable[i].out);
    strcat (result, "=");
    ret = 1;
    break;
  }
     }
 }
      else
 {
   size_t i;
   for (i = 0; i < (sizeof (optable) / sizeof ((optable)[0])); i++)
     {
       len1 = len - 3;
       if ((int) strlen (optable[i].in) == len1
    && memcmp (optable[i].in, opname + 3, len1) == 0)
  {
    strcat (result, "operator");
    strcat (result, optable[i].out);
    ret = 1;
    break;
  }
     }
 }
    }
  else if (len >= 5 && memcmp (opname, "type", 4) == 0
    && strchr (cplus_markers, opname[4]) != ((void *)0))
    {
      tem = opname + 5;
      if (do_type (work, &tem, &type))
 {
   strcat (result, "operator ");
   strncat (result, type.b, type.p - type.b);
   string_delete (&type);
   ret = 1;
 }
    }
  squangle_mop_up (work);
  return ret;
}
const char *
cplus_mangle_opname (opname, options)
     const char *opname;
     int options;
{
  size_t i;
  int len;
  len = strlen (opname);
  for (i = 0; i < (sizeof (optable) / sizeof ((optable)[0])); i++)
    {
      if ((int) strlen (optable[i].out) == len
   && (options & (1 << 1)) == (optable[i].flags & (1 << 1))
   && memcmp (optable[i].out, opname, len) == 0)
 return optable[i].in;
    }
  return (0);
}
enum demangling_styles
cplus_demangle_set_style (style)
     enum demangling_styles style;
{
  const struct demangler_engine *demangler = libiberty_demanglers;
  for (; demangler->demangling_style != unknown_demangling; ++demangler)
    if (style == demangler->demangling_style)
      {
 current_demangling_style = style;
 return current_demangling_style;
      }
  return unknown_demangling;
}
enum demangling_styles
cplus_demangle_name_to_style (name)
     const char *name;
{
  const struct demangler_engine *demangler = libiberty_demanglers;
  for (; demangler->demangling_style != unknown_demangling; ++demangler)
    if (strcmp (name, demangler->demangling_style_name) == 0)
      return demangler->demangling_style;
  return unknown_demangling;
}
char *
cplus_demangle (mangled, options)
     const char *mangled;
     int options;
{
  char *ret;
  struct work_stuff work[1];
  if (current_demangling_style == no_demangling)
    return xstrdup (mangled);
  memset ((char *) work, 0, sizeof (work));
  work->options = options;
  if ((work->options & ((1 << 8)|(1 << 9)|(1 << 10)|(1 << 11)|(1 << 12)|(1 << 13)|(1 << 14)|(1 << 2)|(1 << 15))) == 0)
    work->options |= (int) current_demangling_style & ((1 << 8)|(1 << 9)|(1 << 10)|(1 << 11)|(1 << 12)|(1 << 13)|(1 << 14)|(1 << 2)|(1 << 15));
  if ((((int) work->options) & (1 << 14)) || (((int) work->options) & (1 << 8)))
    {
      ret = cplus_demangle_v3 (mangled, work->options);
      if (ret || (((int) work->options) & (1 << 14)))
 return ret;
    }
  if ((((int) work->options) & (1 << 2)))
    {
      ret = java_demangle_v3 (mangled);
      if (ret)
        return ret;
    }
  if ((((int) work->options) & (1 << 15)))
    return ada_demangle(mangled,options);
  ret = internal_cplus_demangle (work, mangled);
  squangle_mop_up (work);
  return (ret);
}
static void
grow_vect (old_vect, size, min_size, element_size)
     char *old_vect;
     size_t *size;
     size_t min_size;
     int element_size;
{
  if (*size < min_size)
    {
      *size *= 2;
      if (*size < min_size)
 *size = min_size;
      *old_vect = (void *) xrealloc (*old_vect, *size * element_size);
    }
}
static char *
ada_demangle (mangled, option)
     const char *mangled;
     int option ;
{
  int i, j;
  int len0;
  const char* p;
  char *demangled = ((void *)0);
  int at_start_name;
  int changed;
  size_t demangled_size = 0;
  changed = 0;
  if (strncmp (mangled, "_ada_", 5) == 0)
    {
      mangled += 5;
      changed = 1;
    }
  if (mangled[0] == '_' || mangled[0] == '<')
    goto Suppress;
  p = strstr (mangled, "___");
  if (p == ((void *)0))
    len0 = strlen (mangled);
  else
    {
      if (p[3] == 'X')
 {
   len0 = p - mangled;
   changed = 1;
 }
      else
 goto Suppress;
    }
  grow_vect (&demangled,
      &demangled_size, 2 * len0 + 1,
      sizeof (char));
  if ((_sch_istable[((unsigned char) mangled[len0 - 1]) & 0xff] & (unsigned short)(_sch_isdigit))) {
    for (i = len0 - 2; i >= 0 && (_sch_istable[((unsigned char) mangled[i]) & 0xff] & (unsigned short)(_sch_isdigit)); i -= 1)
      ;
    if (i > 1 && mangled[i] == '_' && mangled[i - 1] == '_')
      {
 len0 = i - 1;
 changed = 1;
      }
    else if (mangled[i] == '$')
      {
 len0 = i;
 changed = 1;
      }
  }
  for (i = 0, j = 0; i < len0 && ! (_sch_istable[((unsigned char)mangled[i]) & 0xff] & (unsigned short)(_sch_isalpha));
       i += 1, j += 1)
    demangled[j] = mangled[i];
  at_start_name = 1;
  while (i < len0)
    {
      at_start_name = 0;
      if (i < len0 - 2 && mangled[i] == '_' && mangled[i + 1] == '_')
 {
   demangled[j] = '.';
   changed = at_start_name = 1;
   i += 2; j += 1;
 }
      else
 {
   demangled[j] = mangled[i];
   i += 1; j += 1;
 }
    }
  demangled[j] = 0;
  for (i = 0; demangled[i] != 0; i += 1)
    if ((_sch_istable[((unsigned char)demangled[i]) & 0xff] & (unsigned short)(_sch_isupper)) || demangled[i] == ' ')
      goto Suppress;
  if (! changed)
    return ((void *)0);
  else
    return demangled;
 Suppress:
  grow_vect (&demangled,
      &demangled_size, strlen (mangled) + 3,
      sizeof (char));
  if (mangled[0] == '<')
     strcpy (demangled, mangled);
  else
    sprintf (demangled, "<%s>", mangled);
  return demangled;
}
static char *
internal_cplus_demangle (work, mangled)
     struct work_stuff *work;
     const char *mangled;
{
  string decl;
  int success = 0;
  char *demangled = ((void *)0);
  int s1, s2, s3, s4;
  s1 = work->constructor;
  s2 = work->destructor;
  s3 = work->static_type;
  s4 = work->type_quals;
  work->constructor = work->destructor = 0;
  work->type_quals = 0x0;
  work->dllimported = 0;
  if ((mangled != ((void *)0)) && (*mangled != 0))
    {
      string_init (&decl);
      if (((((int) work->options) & (1 << 8)) || (((int) work->options) & (1 << 9))))
 {
   success = gnu_special (work, &mangled, &decl);
 }
      if (!success)
 {
   success = demangle_prefix (work, &mangled, &decl);
 }
      if (success && (*mangled != 0))
 {
   success = demangle_signature (work, &mangled, &decl);
 }
      if (work->constructor == 2)
        {
          string_prepend (&decl, "global constructors keyed to ");
          work->constructor = 0;
        }
      else if (work->destructor == 2)
        {
          string_prepend (&decl, "global destructors keyed to ");
          work->destructor = 0;
        }
      else if (work->dllimported == 1)
        {
          string_prepend (&decl, "import stub for ");
          work->dllimported = 0;
        }
      demangled = mop_up (work, &decl, success);
    }
  work->constructor = s1;
  work->destructor = s2;
  work->static_type = s3;
  work->type_quals = s4;
  return demangled;
}
static void
squangle_mop_up (work)
     struct work_stuff *work;
{
  forget_B_and_K_types (work);
  if (work -> btypevec != ((void *)0))
    {
      free ((char *) work -> btypevec);
    }
  if (work -> ktypevec != ((void *)0))
    {
      free ((char *) work -> ktypevec);
    }
}
static void
work_stuff_copy_to_from (to, from)
     struct work_stuff *to;
     struct work_stuff *from;
{
  int i;
  delete_work_stuff (to);
  memcpy (to, from, sizeof (*to));
  if (from->typevec_size)
    to->typevec
      = (char *) xmalloc (from->typevec_size * sizeof (to->typevec[0]));
  for (i = 0; i < from->ntypes; i++)
    {
      int len = strlen (from->typevec[i]) + 1;
      to->typevec[i] = xmalloc (len);
      memcpy (to->typevec[i], from->typevec[i], len);
    }
  if (from->ksize)
    to->ktypevec
      = (char *) xmalloc (from->ksize * sizeof (to->ktypevec[0]));
  for (i = 0; i < from->numk; i++)
    {
      int len = strlen (from->ktypevec[i]) + 1;
      to->ktypevec[i] = xmalloc (len);
      memcpy (to->ktypevec[i], from->ktypevec[i], len);
    }
  if (from->bsize)
    to->btypevec
      = (char *) xmalloc (from->bsize * sizeof (to->btypevec[0]));
  for (i = 0; i < from->numb; i++)
    {
      int len = strlen (from->btypevec[i]) + 1;
      to->btypevec[i] = xmalloc (len);
      memcpy (to->btypevec[i], from->btypevec[i], len);
    }
  if (from->ntmpl_args)
    to->tmpl_argvec
      = (char *) xmalloc (from->ntmpl_args * sizeof (to->tmpl_argvec[0]));
  for (i = 0; i < from->ntmpl_args; i++)
    {
      int len = strlen (from->tmpl_argvec[i]) + 1;
      to->tmpl_argvec[i] = xmalloc (len);
      memcpy (to->tmpl_argvec[i], from->tmpl_argvec[i], len);
    }
  if (from->previous_argument)
    {
      to->previous_argument = (string*) xmalloc (sizeof (string));
      string_init (to->previous_argument);
      string_appends (to->previous_argument, from->previous_argument);
    }
}
static void
delete_non_B_K_work_stuff (work)
     struct work_stuff *work;
{
  forget_types (work);
  if (work -> typevec != ((void *)0))
    {
      free ((char *) work -> typevec);
      work -> typevec = ((void *)0);
      work -> typevec_size = 0;
    }
  if (work->tmpl_argvec)
    {
      int i;
      for (i = 0; i < work->ntmpl_args; i++)
 if (work->tmpl_argvec[i])
   free ((char*) work->tmpl_argvec[i]);
      free ((char*) work->tmpl_argvec);
      work->tmpl_argvec = ((void *)0);
    }
  if (work->previous_argument)
    {
      string_delete (work->previous_argument);
      free ((char*) work->previous_argument);
      work->previous_argument = ((void *)0);
    }
}
static void
delete_work_stuff (work)
     struct work_stuff *work;
{
  delete_non_B_K_work_stuff (work);
  squangle_mop_up (work);
}
static char *
mop_up (work, declp, success)
     struct work_stuff *work;
     string *declp;
     int success;
{
  char *demangled = ((void *)0);
  delete_non_B_K_work_stuff (work);
  if (!success)
    {
      string_delete (declp);
    }
  else
    {
      string_appendn (declp, "", 1);
      demangled = declp->b;
    }
  return (demangled);
}
static int
demangle_signature (work, mangled, declp)
     struct work_stuff *work;
     const char *mangled;
     string *declp;
{
  int success = 1;
  int func_done = 0;
  int expect_func = 0;
  int expect_return_type = 0;
  const char *oldmangled = ((void *)0);
  string trawname;
  string tname;
  while (success && (*mangled != 0))
    {
      switch (*mangled)
 {
 case 'Q':
   oldmangled = *mangled;
   success = demangle_qualified (work, mangled, declp, 1, 0);
   if (success)
     remember_type (work, oldmangled, *mangled - oldmangled);
   if ((((int) work->options) & (1 << 8)) || (((int) work->options) & (1 << 9)))
     expect_func = 1;
   oldmangled = ((void *)0);
   break;
        case 'K':
   oldmangled = *mangled;
   success = demangle_qualified (work, mangled, declp, 1, 0);
   if ((((int) work->options) & (1 << 8)) || (((int) work->options) & (1 << 9)))
     {
       expect_func = 1;
     }
   oldmangled = ((void *)0);
   break;
 case 'S':
   if (oldmangled == ((void *)0))
     {
       oldmangled = *mangled;
     }
   (*mangled)++;
   work -> static_type = 1;
   break;
 case 'C':
 case 'V':
 case 'u':
   work->type_quals |= code_for_qualifier (*mangled);
   if (oldmangled == ((void *)0))
     oldmangled = *mangled;
   (*mangled)++;
   break;
 case 'L':
   if ((((int) work->options) & (1 << 12)))
     {
       while (*mangled && (*mangled != '_'))
  (*mangled)++;
       if (!*mangled)
  success = 0;
       else
  (*mangled)++;
     }
   else
     success = 0;
   break;
 case '0': case '1': case '2': case '3': case '4':
 case '5': case '6': case '7': case '8': case '9':
   if (oldmangled == ((void *)0))
     {
       oldmangled = *mangled;
     }
          work->temp_start = -1;
   success = demangle_class (work, mangled, declp);
   if (success)
     {
       remember_type (work, oldmangled, *mangled - oldmangled);
     }
   if ((((int) work->options) & (1 << 8)) || (((int) work->options) & (1 << 9)) || (((int) work->options) & (1 << 13)))
     {
              if (*mangled != 'F')
                 expect_func = 1;
     }
   oldmangled = ((void *)0);
   break;
 case 'B':
   {
     string s;
     success = do_type (work, mangled, &s);
     if (success)
       {
  string_append (&s, ((work->options & (1 << 2)) ? "." : "::"));
  string_prepends (declp, &s);
  string_delete (&s);
       }
     oldmangled = ((void *)0);
     expect_func = 1;
   }
   break;
 case 'F':
   oldmangled = ((void *)0);
   func_done = 1;
   (*mangled)++;
   if ((((int) work->options) & (1 << 10)) || (((int) work->options) & (1 << 11)) || (((int) work->options) & (1 << 12)) || (((int) work->options) & (1 << 13)))
     {
       forget_types (work);
     }
   success = demangle_args (work, mangled, declp);
   if (success && ((((int) work->options) & (1 << 8)) || (((int) work->options) & (1 << 13))) && *mangled == '_')
     {
       ++(*mangled);
              success = do_type (work, mangled, &tname);
              string_delete (&tname);
            }
   break;
 case 't':
   string_init(&trawname);
   string_init(&tname);
   if (oldmangled == ((void *)0))
     {
       oldmangled = *mangled;
     }
   success = demangle_template (work, mangled, &tname,
           &trawname, 1, 1);
   if (success)
     {
       remember_type (work, oldmangled, *mangled - oldmangled);
     }
   string_append (&tname, ((work->options & (1 << 2)) ? "." : "::"));
   string_prepends(declp, &tname);
   if (work -> destructor & 1)
     {
       string_prepend (&trawname, "~");
       string_appends (declp, &trawname);
       work->destructor -= 1;
     }
   if ((work->constructor & 1) || (work->destructor & 1))
     {
       string_appends (declp, &trawname);
       work->constructor -= 1;
     }
   string_delete(&trawname);
   string_delete(&tname);
   oldmangled = ((void *)0);
   expect_func = 1;
   break;
 case '_':
   if (((((int) work->options) & (1 << 8)) || (((int) work->options) & (1 << 9))) && expect_return_type)
     {
       string return_type;
       (*mangled)++;
       success = do_type (work, mangled, &return_type);
       {if (!((&return_type) -> b == (&return_type) -> p)) string_append(&return_type, " ");};
       string_prepends (declp, &return_type);
       string_delete (&return_type);
       break;
     }
   else
            if ((((int) work->options) & (1 << 12)))
              {
                (*mangled)++;
                while (*mangled && (_sch_istable[((unsigned char)*mangled) & 0xff] & (unsigned short)(_sch_isdigit)))
                  (*mangled)++;
              }
            else
       success = 0;
   break;
 case 'H':
   if ((((int) work->options) & (1 << 8)) || (((int) work->options) & (1 << 9)))
     {
       success = demangle_template (work, mangled, declp, 0, 0,
        0);
       if (!(work->constructor & 1))
  expect_return_type = 1;
       (*mangled)++;
       break;
     }
   else
     {;}
 default:
   if ((((int) work->options) & (1 << 8)) || (((int) work->options) & (1 << 9)))
     {
       func_done = 1;
       success = demangle_args (work, mangled, declp);
     }
   else
     {
       success = 0;
     }
   break;
 }
      {
 if (success && expect_func)
   {
     func_done = 1;
              if ((((int) work->options) & (1 << 10)) || (((int) work->options) & (1 << 11)) || (((int) work->options) & (1 << 13)))
                {
                  forget_types (work);
                }
     success = demangle_args (work, mangled, declp);
     expect_func = 0;
   }
      }
    }
  if (success && !func_done)
    {
      if ((((int) work->options) & (1 << 8)) || (((int) work->options) & (1 << 9)))
 {
   success = demangle_args (work, mangled, declp);
 }
    }
  if (success && (work -> options & (1 << 0)))
    {
      if (work->static_type)
 string_append (declp, " static");
      if (work->type_quals != 0x0)
 {
   {if (!((declp) -> b == (declp) -> p)) string_append(declp, " ");};
   string_append (declp, qualifier_string (work->type_quals));
 }
    }
  return (success);
}
static int
demangle_template_template_parm (work, mangled, tname)
     struct work_stuff *work;
     const char *mangled;
     string *tname;
{
  int i;
  int r;
  int need_comma = 0;
  int success = 1;
  string temp;
  string_append (tname, "template <");
  if (get_count (mangled, &r))
    {
      for (i = 0; i < r; i++)
 {
   if (need_comma)
     {
       string_append (tname, ", ");
     }
     if (*mangled == 'Z')
       {
  (*mangled)++;
  string_append (tname, "class");
       }
     else if (*mangled == 'z')
       {
  (*mangled)++;
  success =
    demangle_template_template_parm (work, mangled, tname);
  if (!success)
    {
      break;
    }
       }
     else
       {
  success = do_type (work, mangled, &temp);
  if (success)
    {
      string_appends (tname, &temp);
    }
  string_delete(&temp);
  if (!success)
    {
      break;
    }
       }
   need_comma = 1;
 }
    }
  if (tname->p[-1] == '>')
    string_append (tname, " ");
  string_append (tname, "> class");
  return (success);
}
static int
demangle_expression (work, mangled, s, tk)
     struct work_stuff *work;
     const char* mangled;
     string* s;
     type_kind_t tk;
{
  int need_operator = 0;
  int success;
  success = 1;
  string_appendn (s, "(", 1);
  (*mangled)++;
  while (success && *mangled != 'W' && *mangled != 0)
    {
      if (need_operator)
 {
   size_t i;
   size_t len;
   success = 0;
   len = strlen (*mangled);
   for (i = 0; i < (sizeof (optable) / sizeof ((optable)[0])); ++i)
     {
       size_t l = strlen (optable[i].in);
       if (l <= len
    && memcmp (optable[i].in, *mangled, l) == 0)
  {
    string_appendn (s, " ", 1);
    string_append (s, optable[i].out);
    string_appendn (s, " ", 1);
    success = 1;
    (*mangled) += l;
    break;
  }
     }
   if (!success)
     break;
 }
      else
 need_operator = 1;
      success = demangle_template_value_parm (work, mangled, s, tk);
    }
  if (*mangled != 'W')
    success = 0;
  else
    {
      string_appendn (s, ")", 1);
      (*mangled)++;
    }
  return success;
}
static int
demangle_integral_value (work, mangled, s)
     struct work_stuff *work;
     const char* mangled;
     string* s;
{
  int success;
  if (*mangled == 'E')
    success = demangle_expression (work, mangled, s, tk_integral);
  else if (*mangled == 'Q' || *mangled == 'K')
    success = demangle_qualified (work, mangled, s, 0, 1);
  else
    {
      int value1;
      int multidigit_without_leading_underscore = 0;
      int leave_following_underscore = 0;
      success = 0;
      if (*mangled == '_')
        {
   if (mangled[0][1] == 'm')
     {
       multidigit_without_leading_underscore = 1;
       string_appendn (s, "-", 1);
       (*mangled) += 2;
     }
   else
     {
       leave_following_underscore = 1;
     }
 }
      else
 {
   if (*mangled == 'm')
   {
     string_appendn (s, "-", 1);
     (*mangled)++;
   }
   multidigit_without_leading_underscore = 1;
   leave_following_underscore = 1;
 }
      if (multidigit_without_leading_underscore)
 value = consume_count (mangled);
      else
 value = consume_count_with_underscores (mangled);
      if (value != -1)
 {
   char buf[32];
   sprintf (buf, "%d", value);
   string_append (s, buf);
   if ((value > 9 || multidigit_without_leading_underscore)
       && ! leave_following_underscore
       && *mangled == '_')
     (*mangled)++;
   success = 1;
 }
      }
  return success;
}
static int
demangle_real_value (work, mangled, s)
     struct work_stuff *work;
     const char *mangled;
     string* s;
{
  if (*mangled == 'E')
    return demangle_expression (work, mangled, s, tk_real);
  if (*mangled == 'm')
    {
      string_appendn (s, "-", 1);
      (*mangled)++;
    }
  while ((_sch_istable[((unsigned char)*mangled) & 0xff] & (unsigned short)(_sch_isdigit)))
    {
      string_appendn (s, *mangled, 1);
      (*mangled)++;
    }
  if (*mangled == '.')
    {
      string_appendn (s, ".", 1);
      (*mangled)++;
      while ((_sch_istable[((unsigned char)*mangled) & 0xff] & (unsigned short)(_sch_isdigit)))
 {
   string_appendn (s, *mangled, 1);
   (*mangled)++;
 }
    }
  if (*mangled == 'e')
    {
      string_appendn (s, "e", 1);
      (*mangled)++;
      while ((_sch_istable[((unsigned char)*mangled) & 0xff] & (unsigned short)(_sch_isdigit)))
 {
   string_appendn (s, *mangled, 1);
   (*mangled)++;
 }
    }
  return 1;
}
static int
demangle_template_value_parm (work, mangled, s, tk)
     struct work_stuff *work;
     const char *mangled;
     string* s;
     type_kind_t tk;
{
  int success = 1;
  if (*mangled == 'Y')
    {
      int idx;
      (*mangled)++;
      idx = consume_count_with_underscores (mangled);
      if (idx == -1
   || (work->tmpl_argvec && idx >= work->ntmpl_args)
   || consume_count_with_underscores (mangled) == -1)
 return -1;
      if (work->tmpl_argvec)
 string_append (s, work->tmpl_argvec[idx]);
      else
 string_append_template_idx (s, idx);
    }
  else if (tk == tk_integral)
    success = demangle_integral_value (work, mangled, s);
  else if (tk == tk_char)
    {
      char tmp[2];
      int val;
      if (*mangled == 'm')
 {
   string_appendn (s, "-", 1);
   (*mangled)++;
 }
      string_appendn (s, "'", 1);
      val = consume_count(mangled);
      if (val <= 0)
 success = 0;
      else
 {
   tmp[0] = (char)val;
   tmp[1] = 0;
   string_appendn (s, &tmp[0], 1);
   string_appendn (s, "'", 1);
 }
    }
  else if (tk == tk_bool)
    {
      int val = consume_count (mangled);
      if (val == 0)
 string_appendn (s, "false", 5);
      else if (val == 1)
 string_appendn (s, "true", 4);
      else
 success = 0;
    }
  else if (tk == tk_real)
    success = demangle_real_value (work, mangled, s);
  else if (tk == tk_pointer || tk == tk_reference)
    {
      if (*mangled == 'Q')
 success = demangle_qualified (work, mangled, s,
                         0,
                     1);
      else
 {
   int symbol_len = consume_count (mangled);
   if (symbol_len == -1)
     return -1;
   if (symbol_len == 0)
     string_appendn (s, "0", 1);
   else
     {
       char *p = xmalloc (symbol_len + 1), *q;
       strncpy (p, *mangled, symbol_len);
       p [symbol_len] = 0;
       q = cplus_demangle (p, work->options);
       if (tk == tk_pointer)
  string_appendn (s, "&", 1);
       if (q)
  {
    string_append (s, q);
    free (q);
  }
       else
  string_append (s, p);
       free (p);
     }
   *mangled += symbol_len;
 }
    }
  return success;
}
static int
demangle_template (work, mangled, tname, trawname, is_type, remember)
     struct work_stuff *work;
     const char *mangled;
     string *tname;
     string *trawname;
     int is_type;
     int remember;
{
  int i;
  int r;
  int need_comma = 0;
  int success = 0;
  const char *start;
  int is_java_array = 0;
  string temp;
  (*mangled)++;
  if (is_type)
    {
      start = *mangled;
      if (*mangled == 'z')
 {
   int idx;
   (*mangled)++;
   (*mangled)++;
   idx = consume_count_with_underscores (mangled);
   if (idx == -1
       || (work->tmpl_argvec && idx >= work->ntmpl_args)
       || consume_count_with_underscores (mangled) == -1)
     return (0);
   if (work->tmpl_argvec)
     {
       string_append (tname, work->tmpl_argvec[idx]);
       if (trawname)
  string_append (trawname, work->tmpl_argvec[idx]);
     }
   else
     {
       string_append_template_idx (tname, idx);
       if (trawname)
  string_append_template_idx (trawname, idx);
     }
 }
      else
 {
   if ((r = consume_count (mangled)) <= 0
       || (int) strlen (*mangled) < r)
     {
       return (0);
     }
   is_java_array = (work -> options & (1 << 2))
     && strncmp (*mangled, "JArray1Z", 8) == 0;
   if (! is_java_array)
     {
       string_appendn (tname, *mangled, r);
     }
   if (trawname)
     string_appendn (trawname, *mangled, r);
   *mangled += r;
 }
    }
  if (!is_java_array)
    string_append (tname, "<");
  if (!get_count (mangled, &r))
    {
      return (0);
    }
  if (!is_type)
    {
      work->tmpl_argvec = (char*) xmalloc (r * sizeof (char *));
      work->ntmpl_args = r;
      for (i = 0; i < r; i++)
 work->tmpl_argvec[i] = 0;
    }
  for (i = 0; i < r; i++)
    {
      if (need_comma)
 {
   string_append (tname, ", ");
 }
      if (*mangled == 'Z')
 {
   (*mangled)++;
   success = do_type (work, mangled, &temp);
   if (success)
     {
       string_appends (tname, &temp);
       if (!is_type)
  {
    int len = temp.p - temp.b;
    work->tmpl_argvec[i] = xmalloc (len + 1);
    memcpy (work->tmpl_argvec[i], temp.b, len);
    work->tmpl_argvec[i][len] = 0;
  }
     }
   string_delete(&temp);
   if (!success)
     {
       break;
     }
 }
      else if (*mangled == 'z')
 {
   int r2;
   (*mangled)++;
   success = demangle_template_template_parm (work, mangled, tname);
   if (success
       && (r2 = consume_count (mangled)) > 0
       && (int) strlen (*mangled) >= r2)
     {
       string_append (tname, " ");
       string_appendn (tname, *mangled, r2);
       if (!is_type)
  {
    int len = r2;
    work->tmpl_argvec[i] = xmalloc (len + 1);
    memcpy (work->tmpl_argvec[i], *mangled, len);
    work->tmpl_argvec[i][len] = 0;
  }
       *mangled += r2;
     }
   if (!success)
     {
       break;
     }
 }
      else
 {
   string param;
   string* s;
   success = do_type (work, mangled, &temp);
   string_delete(&temp);
   if (!success)
     break;
   if (!is_type)
     {
       s = &param;
       string_init (s);
     }
   else
     s = tname;
   success = demangle_template_value_parm (work, mangled, s,
        (type_kind_t) success);
   if (!success)
     {
       if (!is_type)
  string_delete (s);
       success = 0;
       break;
     }
   if (!is_type)
     {
       int len = s->p - s->b;
       work->tmpl_argvec[i] = xmalloc (len + 1);
       memcpy (work->tmpl_argvec[i], s->b, len);
       work->tmpl_argvec[i][len] = 0;
       string_appends (tname, s);
       string_delete (s);
     }
 }
      need_comma = 1;
    }
  if (is_java_array)
    {
      string_append (tname, "[]");
    }
  else
    {
      if (tname->p[-1] == '>')
 string_append (tname, " ");
      string_append (tname, ">");
    }
  if (is_type && remember)
    {
      const int bindex = register_Btype (work);
      remember_Btype (work, tname->b, ( (((tname) -> b == (tname) -> p))?0:((tname)->p - (tname)->b)), bindex);
    }
  return (success);
}
static int
arm_pt (work, mangled, n, anchor, args)
     struct work_stuff *work;
     const char *mangled;
     int n;
     const char *anchor, *args;
{
  if (((((int) work->options) & (1 << 11)) || (((int) work->options) & (1 << 12))) && (*anchor = strstr (mangled, "__pt__")))
    {
      int len;
      *args = *anchor + 6;
      len = consume_count (args);
      if (len == -1)
 return 0;
      if (*args + len == mangled + n && *args == '_')
 {
   ++*args;
   return 1;
 }
    }
  if ((((int) work->options) & (1 << 8)) || (((int) work->options) & (1 << 13)))
    {
      if ((*anchor = strstr (mangled, "__tm__"))
          || (*anchor = strstr (mangled, "__ps__"))
          || (*anchor = strstr (mangled, "__pt__")))
        {
          int len;
          *args = *anchor + 6;
          len = consume_count (args);
   if (len == -1)
     return 0;
          if (*args + len == mangled + n && *args == '_')
            {
              ++*args;
              return 1;
            }
        }
      else if ((*anchor = strstr (mangled, "__S")))
        {
    int len;
    *args = *anchor + 3;
    len = consume_count (args);
   if (len == -1)
     return 0;
    if (*args + len == mangled + n && *args == '_')
            {
              ++*args;
        return 1;
            }
        }
    }
  return 0;
}
static void
demangle_arm_hp_template (work, mangled, n, declp)
     struct work_stuff *work;
     const char *mangled;
     int n;
     string *declp;
{
  const char *p;
  const char *args;
  const char *e = *mangled + n;
  string arg;
  if ((((int) work->options) & (1 << 12)) && ((*mangled)[n] == 'X'))
    {
      char *start_spec_args = ((void *)0);
      int hold_options;
      start_spec_args = strchr (*mangled, '<');
      if (start_spec_args && (start_spec_args - *mangled < n))
        string_appendn (declp, *mangled, start_spec_args - *mangled);
      else
        string_appendn (declp, *mangled, n);
      (*mangled) += n + 1;
      string_init (&arg);
      if (work->temp_start == -1)
        work->temp_start = declp->p - declp->b;
      hold_options = work->options;
      work->options |= (1 << 0);
      string_append (declp, "<");
      while (1)
        {
          string_delete (&arg);
          switch (*mangled)
            {
              case 'T':
                (*mangled)++;
                if (!do_type (work, mangled, &arg))
                  goto hpacc_template_args_done;
                break;
              case 'U':
              case 'S':
                if (!do_hpacc_template_const_value (work, mangled, &arg))
                  goto hpacc_template_args_done;
                break;
              case 'A':
                if (!do_hpacc_template_literal (work, mangled, &arg))
                  goto hpacc_template_args_done;
                break;
              default:
                goto hpacc_template_args_done;
            }
          string_appends (declp, &arg);
          if ((*mangled == 0) || (*mangled == '_'))
            break;
          else
            string_append (declp, ",");
        }
    hpacc_template_args_done:
      string_append (declp, ">");
      string_delete (&arg);
      if (*mangled == '_')
        (*mangled)++;
      work->options = hold_options;
      return;
    }
  else if (arm_pt (work, *mangled, n, &p, &args))
    {
      int hold_options;
      string type_str;
      string_init (&arg);
      string_appendn (declp, *mangled, p - *mangled);
      if (work->temp_start == -1)
 work->temp_start = declp->p - declp->b;
      hold_options = work->options;
      work->options |= (1 << 0);
      string_append (declp, "<");
      while (args < e) {
 string_delete (&arg);
 switch (*args)
   {
          case 'X':
            args++;
            if (!do_type (work, &args, &type_str))
       goto cfront_template_args_done;
            string_append (&arg, "(");
            string_appends (&arg, &type_str);
            string_delete (&type_str);
            string_append (&arg, ")");
            if (*args != 'L')
              goto cfront_template_args_done;
            args++;
            if (!snarf_numeric_literal (&args, &arg))
       goto cfront_template_args_done;
            break;
          case 'L':
            args++;
            if (!snarf_numeric_literal (&args, &arg))
       goto cfront_template_args_done;
            break;
          default:
            {
              const char* old_args = args;
              if (!do_type (work, &args, &arg))
                goto cfront_template_args_done;
              if (args == old_args)
  {
    work->options = hold_options;
    return;
  }
            }
   }
 string_appends (declp, &arg);
 string_append (declp, ",");
      }
    cfront_template_args_done:
      string_delete (&arg);
      if (args >= e)
 --declp->p;
      string_append (declp, ">");
      work->options = hold_options;
    }
  else if (n>10 && strncmp (*mangled, "_GLOBAL_", 8) == 0
    && (*mangled)[9] == 'N'
    && (*mangled)[8] == (*mangled)[10]
    && strchr (cplus_markers, (*mangled)[8]))
    {
      string_append (declp, "{anonymous}");
    }
  else
    {
      if (work->temp_start == -1)
 work->temp_start = 0;
      string_appendn (declp, *mangled, n);
    }
  *mangled += n;
}
static int
demangle_class_name (work, mangled, declp)
     struct work_stuff *work;
     const char *mangled;
     string *declp;
{
  int n;
  int success = 0;
  n = consume_count (mangled);
  if (n == -1)
    return 0;
  if ((int) strlen (*mangled) >= n)
    {
      demangle_arm_hp_template (work, mangled, n, declp);
      success = 1;
    }
  return (success);
}
static int
demangle_class (work, mangled, declp)
     struct work_stuff *work;
     const char *mangled;
     string *declp;
{
  int success = 0;
  int btype;
  string class_name;
  char *save_class_name_end = 0;
  string_init (&class_name);
  btype = register_Btype (work);
  if (demangle_class_name (work, mangled, &class_name))
    {
      save_class_name_end = class_name.p;
      if ((work->constructor & 1) || (work->destructor & 1))
 {
          if (work->temp_start && (work->temp_start != -1))
            {
              class_name.p = class_name.b + work->temp_start;
            }
   string_prepends (declp, &class_name);
   if (work -> destructor & 1)
     {
       string_prepend (declp, "~");
              work -> destructor -= 1;
     }
   else
     {
       work -> constructor -= 1;
     }
 }
      class_name.p = save_class_name_end;
      remember_Ktype (work, class_name.b, ( (((&class_name) -> b == (&class_name) -> p))?0:((&class_name)->p - (&class_name)->b)));
      remember_Btype (work, class_name.b, ( (((&class_name) -> b == (&class_name) -> p))?0:((&class_name)->p - (&class_name)->b)), btype);
      string_prepend (declp, ((work->options & (1 << 2)) ? "." : "::"));
      string_prepends (declp, &class_name);
      success = 1;
    }
  string_delete (&class_name);
  return (success);
}
static int
iterate_demangle_function (work, mangled, declp, scan)
     struct work_stuff *work;
     const char *mangled;
     string *declp;
     const char *scan;
{
  const char *mangle_init = *mangled;
  int success = 0;
  string decl_init;
  struct work_stuff work_init;
  if (*(scan + 2) == 0)
    return 0;
  if ((((int) work->options) & (1 << 11)) || (((int) work->options) & (1 << 10)) || (((int) work->options) & (1 << 12)) || (((int) work->options) & (1 << 13))
      || strstr (scan + 2, "__") == ((void *)0))
    {
      demangle_function_name (work, mangled, declp, scan);
      return 1;
    }
  string_init (&decl_init);
  string_appends (&decl_init, declp);
  memset (&work_init, 0, sizeof work_init);
  work_stuff_copy_to_from (&work_init, work);
  while (scan[2])
    {
      demangle_function_name (work, mangled, declp, scan);
      success = demangle_signature (work, mangled, declp);
      if (success)
 break;
      *mangled = mangle_init;
      string_clear (declp);
      string_appends (declp, &decl_init);
      work_stuff_copy_to_from (work, &work_init);
      scan += 2;
      while (*scan && (scan[0] != '_' || scan[1] != '_'))
 scan++;
      while (*scan && *scan == '_')
 scan++;
      scan -= 2;
    }
  delete_work_stuff (&work_init);
  string_delete (&decl_init);
  return success;
}
static int
demangle_prefix (work, mangled, declp)
     struct work_stuff *work;
     const char *mangled;
     string *declp;
{
  int success = 1;
  const char *scan;
  int i;
  if (strlen(*mangled) > 6
      && (strncmp(*mangled, "_imp__", 6) == 0
          || strncmp(*mangled, "__imp_", 6) == 0))
    {
      (*mangled) += 6;
      work->dllimported = 1;
    }
  else if (strlen(*mangled) >= 11 && strncmp(*mangled, "_GLOBAL_", 8) == 0)
    {
      char *marker = strchr (cplus_markers, (*mangled)[8]);
      if (marker != ((void *)0) && *marker == (*mangled)[10])
 {
   if ((*mangled)[9] == 'D')
     {
       (*mangled) += 11;
       work->destructor = 2;
       if (gnu_special (work, mangled, declp))
  return success;
     }
   else if ((*mangled)[9] == 'I')
     {
       (*mangled) += 11;
       work->constructor = 2;
       if (gnu_special (work, mangled, declp))
  return success;
     }
 }
    }
  else if (((((int) work->options) & (1 << 11)) || (((int) work->options) & (1 << 12)) || (((int) work->options) & (1 << 13))) && strncmp(*mangled, "__std__", 7) == 0)
    {
      (*mangled) += 7;
      work->destructor = 2;
    }
  else if (((((int) work->options) & (1 << 11)) || (((int) work->options) & (1 << 12)) || (((int) work->options) & (1 << 13))) && strncmp(*mangled, "__sti__", 7) == 0)
    {
      (*mangled) += 7;
      work->constructor = 2;
    }
  {
    scan = *mangled;
    do {
      scan = strchr (scan, '_');
    } while (scan != ((void *)0) && *++scan != '_');
    if (scan != ((void *)0)) --scan;
  }
  if (scan != ((void *)0))
    {
      i = strspn (scan, "_");
      if (i > 2)
 {
   scan += (i - 2);
 }
    }
  if (scan == ((void *)0))
    {
      success = 0;
    }
  else if (work -> static_type)
    {
      if (!(_sch_istable[((unsigned char)scan[0]) & 0xff] & (unsigned short)(_sch_isdigit)) && (scan[0] != 't'))
 {
   success = 0;
 }
    }
  else if ((scan == *mangled)
    && ((_sch_istable[((unsigned char)scan[2]) & 0xff] & (unsigned short)(_sch_isdigit)) || (scan[2] == 'Q')
        || (scan[2] == 't') || (scan[2] == 'K') || (scan[2] == 'H')))
    {
      if (((((int) work->options) & (1 << 10)) || (((int) work->options) & (1 << 11)) || (((int) work->options) & (1 << 12)))
   && (_sch_istable[((unsigned char)scan[2]) & 0xff] & (unsigned short)(_sch_isdigit)))
 {
   *mangled = scan + 2;
   consume_count (mangled);
   string_append (declp, *mangled);
   *mangled += strlen (*mangled);
   success = 1;
 }
      else
 {
   if (!((((int) work->options) & (1 << 10)) || (((int) work->options) & (1 << 11)) || (((int) work->options) & (1 << 12)) || (((int) work->options) & (1 << 13))))
     work -> constructor += 1;
   *mangled = scan + 2;
 }
    }
  else if ((((int) work->options) & (1 << 11)) && scan[2] == 'p' && scan[3] == 't')
    {
      success = 1;
      demangle_arm_hp_template (work, mangled, strlen (*mangled), declp);
    }
  else if ((((int) work->options) & (1 << 13)) && ((scan[2] == 't' && scan[3] == 'm')
                              || (scan[2] == 'p' && scan[3] == 's')
                              || (scan[2] == 'p' && scan[3] == 't')))
    {
      success = 1;
      demangle_arm_hp_template (work, mangled, strlen (*mangled), declp);
    }
  else if ((scan == *mangled) && !(_sch_istable[((unsigned char)scan[2]) & 0xff] & (unsigned short)(_sch_isdigit))
    && (scan[2] != 't'))
    {
      if (!((((int) work->options) & (1 << 11)) || (((int) work->options) & (1 << 10)) || (((int) work->options) & (1 << 12)) || (((int) work->options) & (1 << 13)))
   || (arm_special (mangled, declp) == 0))
 {
   while (*scan == '_')
     {
       scan++;
     }
   if ((scan = strstr (scan, "__")) == ((void *)0) || (*(scan + 2) == 0))
     {
       success = 0;
     }
   else
     return iterate_demangle_function (work, mangled, declp, scan);
 }
    }
  else if (*(scan + 2) != 0)
    {
      return iterate_demangle_function (work, mangled, declp, scan);
    }
  else
    {
      success = 0;
    }
  if (!success && (work->constructor == 2 || work->destructor == 2))
    {
      string_append (declp, *mangled);
      *mangled += strlen (*mangled);
      success = 1;
    }
  return (success);
}
static int
gnu_special (work, mangled, declp)
     struct work_stuff *work;
     const char *mangled;
     string *declp;
{
  int n;
  int success = 1;
  const char *p;
  if ((*mangled)[0] == '_'
      && strchr (cplus_markers, (*mangled)[1]) != ((void *)0)
      && (*mangled)[2] == '_')
    {
      (*mangled) += 3;
      work -> destructor += 1;
    }
  else if ((*mangled)[0] == '_'
    && (((*mangled)[1] == '_'
  && (*mangled)[2] == 'v'
  && (*mangled)[3] == 't'
  && (*mangled)[4] == '_')
        || ((*mangled)[1] == 'v'
     && (*mangled)[2] == 't'
     && strchr (cplus_markers, (*mangled)[3]) != ((void *)0))))
    {
      if ((*mangled)[2] == 'v')
 (*mangled) += 5;
      else
 (*mangled) += 4;
      while (*mangled != 0)
 {
   switch (*mangled)
     {
     case 'Q':
     case 'K':
       success = demangle_qualified (work, mangled, declp, 0, 1);
       break;
     case 't':
       success = demangle_template (work, mangled, declp, 0, 1,
        1);
       break;
     default:
       if ((_sch_istable[((unsigned char)*mangled[0]) & 0xff] & (unsigned short)(_sch_isdigit)))
  {
    n = consume_count(mangled);
    if (n > (int) strlen (*mangled))
      {
        success = 1;
        break;
      }
  }
       else
  {
    n = strcspn (*mangled, cplus_markers);
  }
       string_appendn (declp, *mangled, n);
       (*mangled) += n;
     }
   p = strpbrk (*mangled, cplus_markers);
   if (success && ((p == ((void *)0)) || (p == *mangled)))
     {
       if (p != ((void *)0))
  {
    string_append (declp, ((work->options & (1 << 2)) ? "." : "::"));
    (*mangled)++;
  }
     }
   else
     {
       success = 0;
       break;
     }
 }
      if (success)
 string_append (declp, " virtual table");
    }
  else if ((*mangled)[0] == '_'
    && (strchr("0123456789Qt", (*mangled)[1]) != ((void *)0))
    && (p = strpbrk (*mangled, cplus_markers)) != ((void *)0))
    {
      (*mangled)++;
      switch (*mangled)
 {
 case 'Q':
 case 'K':
   success = demangle_qualified (work, mangled, declp, 0, 1);
   break;
 case 't':
   success = demangle_template (work, mangled, declp, 0, 1, 1);
   break;
 default:
   n = consume_count (mangled);
   if (n < 0 || n > (long) strlen (*mangled))
     {
       success = 0;
       break;
     }
   if (n > 10 && strncmp (*mangled, "_GLOBAL_", 8) == 0
       && (*mangled)[9] == 'N'
       && (*mangled)[8] == (*mangled)[10]
       && strchr (cplus_markers, (*mangled)[8]))
     {
       string_append (declp, "{anonymous}");
       (*mangled) += n;
       p = strpbrk (*mangled, cplus_markers);
       break;
     }
   string_appendn (declp, *mangled, n);
   (*mangled) += n;
 }
      if (success && (p == *mangled))
 {
   (*mangled)++;
   string_append (declp, ((work->options & (1 << 2)) ? "." : "::"));
   n = strlen (*mangled);
   string_appendn (declp, *mangled, n);
   (*mangled) += n;
 }
      else
 {
   success = 0;
 }
    }
  else if (strncmp (*mangled, "__thunk_", 8) == 0)
    {
      int delta;
      (*mangled) += 8;
      delta = consume_count (mangled);
      if (delta == -1)
 success = 0;
      else
 {
   char *method = internal_cplus_demangle (work, ++*mangled);
   if (method)
     {
       char buf[50];
       sprintf (buf, "virtual function thunk (delta:%d) for ", -delta);
       string_append (declp, buf);
       string_append (declp, method);
       free (method);
       n = strlen (*mangled);
       (*mangled) += n;
     }
   else
     {
       success = 0;
     }
 }
    }
  else if (strncmp (*mangled, "__t", 3) == 0
    && ((*mangled)[3] == 'i' || (*mangled)[3] == 'f'))
    {
      p = (*mangled)[3] == 'i' ? " type_info node" : " type_info function";
      (*mangled) += 4;
      switch (*mangled)
 {
 case 'Q':
 case 'K':
   success = demangle_qualified (work, mangled, declp, 0, 1);
   break;
 case 't':
   success = demangle_template (work, mangled, declp, 0, 1, 1);
   break;
 default:
   success = do_type (work, mangled, declp);
   break;
 }
      if (success && *mangled != 0)
 success = 0;
      if (success)
 string_append (declp, p);
    }
  else
    {
      success = 0;
    }
  return (success);
}
static void
recursively_demangle(work, mangled, result, namelength)
     struct work_stuff *work;
     const char *mangled;
     string *result;
     int namelength;
{
  char * recurse = (char *)((void *)0);
  char * recurse_dem = (char *)((void *)0);
  recurse = (char *) xmalloc (namelength + 1);
  memcpy (recurse, *mangled, namelength);
  recurse[namelength] = 0;
  recurse_dem = cplus_demangle (recurse, work->options);
  if (recurse_dem)
    {
      string_append (result, recurse_dem);
      free (recurse_dem);
    }
  else
    {
      string_appendn (result, *mangled, namelength);
    }
  free (recurse);
  *mangled += namelength;
}
static int
arm_special (mangled, declp)
     const char *mangled;
     string *declp;
{
  int n;
  int success = 1;
  const char *scan;
  if (strncmp (*mangled, "__vtbl__", 8) == 0)
    {
      scan = *mangled + 8;
      while (*scan != 0)
        {
          n = consume_count (&scan);
          if (n == -1)
     {
       return (0);
     }
          scan += n;
          if (scan[0] == '_' && scan[1] == '_')
     {
       scan += 2;
     }
        }
      (*mangled) += 8;
      while (*mangled != 0)
 {
   n = consume_count (mangled);
          if (n == -1
       || n > (long) strlen (*mangled))
     return 0;
   string_prependn (declp, *mangled, n);
   (*mangled) += n;
   if ((*mangled)[0] == '_' && (*mangled)[1] == '_')
     {
       string_prepend (declp, "::");
       (*mangled) += 2;
     }
 }
      string_append (declp, " virtual table");
    }
  else
    {
      success = 0;
    }
  return (success);
}
static int
demangle_qualified (work, mangled, result, isfuncname, append)
     struct work_stuff *work;
     const char *mangled;
     string *result;
     int isfuncname;
     int append;
{
  int qualifiers = 0;
  int success = 1;
  char num[2];
  string temp;
  string last_name;
  int bindex = register_Btype (work);
  isfuncname = (isfuncname
  && ((work->constructor & 1) || (work->destructor & 1)));
  string_init (&temp);
  string_init (&last_name);
  if ((*mangled)[0] == 'K')
    {
      int idx;
      (*mangled)++;
      idx = consume_count_with_underscores (mangled);
      if (idx == -1 || idx >= work -> numk)
        success = 0;
      else
        string_append (&temp, work -> ktypevec[idx]);
    }
  else
    switch ((*mangled)[1])
    {
    case '_':
      (*mangled)++;
      qualifiers = consume_count_with_underscores (mangled);
      if (qualifiers == -1)
 success = 0;
      break;
    case '1':
    case '2':
    case '3':
    case '4':
    case '5':
    case '6':
    case '7':
    case '8':
    case '9':
      num[0] = (*mangled)[1];
      num[1] = 0;
      qualifiers = atoi (num);
      if ((*mangled)[2] == '_')
 {
   (*mangled)++;
 }
      (*mangled) += 2;
      break;
    case '0':
    default:
      success = 0;
    }
  if (!success)
    return success;
  while (qualifiers-- > 0)
    {
      int remember_K = 1;
      string_clear (&last_name);
      if (*mangled[0] == '_')
 (*mangled)++;
      if (*mangled[0] == 't')
 {
   success = demangle_template(work, mangled, &temp,
          &last_name, 1, 0);
   if (!success)
     break;
 }
      else if (*mangled[0] == 'K')
 {
          int idx;
          (*mangled)++;
          idx = consume_count_with_underscores (mangled);
          if (idx == -1 || idx >= work->numk)
            success = 0;
          else
            string_append (&temp, work->ktypevec[idx]);
          remember_K = 0;
   if (!success) break;
 }
      else
 {
   if ((((int) work->options) & (1 << 13)))
            {
       int namelength;
       namelength = consume_count (mangled);
       if (namelength == -1)
  {
    success = 0;
    break;
  }
        recursively_demangle(work, mangled, &temp, namelength);
            }
          else
            {
              string_delete (&last_name);
              success = do_type (work, mangled, &last_name);
              if (!success)
                break;
              string_appends (&temp, &last_name);
            }
 }
      if (remember_K)
 remember_Ktype (work, temp.b, ( (((&temp) -> b == (&temp) -> p))?0:((&temp)->p - (&temp)->b)));
      if (qualifiers > 0)
 string_append (&temp, ((work->options & (1 << 2)) ? "." : "::"));
    }
  remember_Btype (work, temp.b, ( (((&temp) -> b == (&temp) -> p))?0:((&temp)->p - (&temp)->b)), bindex);
  if (isfuncname)
    {
      string_append (&temp, ((work->options & (1 << 2)) ? "." : "::"));
      if (work -> destructor & 1)
 string_append (&temp, "~");
      string_appends (&temp, &last_name);
    }
  if (append)
    string_appends (result, &temp);
  else
    {
      if (!((result) -> b == (result) -> p))
 string_append (&temp, ((work->options & (1 << 2)) ? "." : "::"));
      string_prepends (result, &temp);
    }
  string_delete (&last_name);
  string_delete (&temp);
  return (success);
}
static int
get_count (type, count)
     const char *type;
     int *count;
{
  const char *p;
  int n;
  if (!(_sch_istable[((unsigned char)*type) & 0xff] & (unsigned short)(_sch_isdigit)))
    return (0);
  else
    {
      *count = *type - '0';
      (*type)++;
      if ((_sch_istable[((unsigned char)*type) & 0xff] & (unsigned short)(_sch_isdigit)))
 {
   p = *type;
   n = *count;
   do
     {
       n *= 10;
       n += *p - '0';
       p++;
     }
   while ((_sch_istable[((unsigned char)*p) & 0xff] & (unsigned short)(_sch_isdigit)));
   if (*p == '_')
     {
       *type = p + 1;
       *count = n;
     }
 }
    }
  return (1);
}
static int
do_type (work, mangled, result)
     struct work_stuff *work;
     const char *mangled;
     string *result;
{
  int n;
  int done;
  int success;
  string decl;
  const char *remembered_type;
  int type_quals;
  type_kind_t tk = tk_none;
  string_init (&decl);
  string_init (result);
  done = 0;
  success = 1;
  while (success && !done)
    {
      int member;
      switch (*mangled)
 {
 case 'P':
 case 'p':
   (*mangled)++;
   if (! (work -> options & (1 << 2)))
     string_prepend (&decl, "*");
   if (tk == tk_none)
     tk = tk_pointer;
   break;
 case 'R':
   (*mangled)++;
   string_prepend (&decl, "&");
   if (tk == tk_none)
     tk = tk_reference;
   break;
 case 'A':
   {
     ++(*mangled);
     if (!((&decl) -> b == (&decl) -> p)
  && (decl.b[0] == '*' || decl.b[0] == '&'))
       {
  string_prepend (&decl, "(");
  string_append (&decl, ")");
       }
     string_append (&decl, "[");
     if (*mangled != '_')
       success = demangle_template_value_parm (work, mangled, &decl,
            tk_integral);
     if (*mangled == '_')
       ++(*mangled);
     string_append (&decl, "]");
     break;
   }
 case 'T':
   (*mangled)++;
   if (!get_count (mangled, &n) || n >= work -> ntypes)
     {
       success = 0;
     }
   else
     {
       remembered_type = work -> typevec[n];
       mangled = &remembered_type;
     }
   break;
 case 'F':
   (*mangled)++;
     if (!((&decl) -> b == (&decl) -> p)
  && (decl.b[0] == '*' || decl.b[0] == '&'))
     {
       string_prepend (&decl, "(");
       string_append (&decl, ")");
     }
   if (!demangle_nested_args (work, mangled, &decl)
       || (*mangled != '_' && *mangled != 0))
     {
       success = 0;
       break;
     }
   if (success && (*mangled == '_'))
     (*mangled)++;
   break;
 case 'M':
 case 'O':
   {
     type_quals = 0x0;
     member = *mangled == 'M';
     (*mangled)++;
     string_append (&decl, ")");
     if (*mangled != 'Q')
       string_prepend (&decl, ((work->options & (1 << 2)) ? "." : "::"));
     if ((_sch_istable[((unsigned char)*mangled) & 0xff] & (unsigned short)(_sch_isdigit)))
       {
  n = consume_count (mangled);
  if (n == -1
      || (int) strlen (*mangled) < n)
    {
      success = 0;
      break;
    }
  string_prependn (&decl, *mangled, n);
  *mangled += n;
       }
     else if (*mangled == 'X' || *mangled == 'Y')
       {
  string temp;
  do_type (work, mangled, &temp);
  string_prepends (&decl, &temp);
  string_delete (&temp);
       }
     else if (*mangled == 't')
       {
  string temp;
  string_init (&temp);
  success = demangle_template (work, mangled, &temp,
          ((void *)0), 1, 1);
  if (success)
    {
      string_prependn (&decl, temp.b, temp.p - temp.b);
      string_delete (&temp);
    }
  else
    break;
       }
     else if (*mangled == 'Q')
       {
  success = demangle_qualified (work, mangled, &decl,
                         0,
                      0);
  if (!success)
    break;
       }
     else
       {
  success = 0;
  break;
       }
     string_prepend (&decl, "(");
     if (member)
       {
  switch (*mangled)
    {
    case 'C':
    case 'V':
    case 'u':
      type_quals |= code_for_qualifier (*mangled);
      (*mangled)++;
      break;
    default:
      break;
    }
  if (*(*mangled)++ != 'F')
    {
      success = 0;
      break;
    }
       }
     if ((member && !demangle_nested_args (work, mangled, &decl))
  || *mangled != '_')
       {
  success = 0;
  break;
       }
     (*mangled)++;
     if (! (work -> options & (1 << 1)))
       {
  break;
       }
     if (type_quals != 0x0)
       {
  {if (!((&decl) -> b == (&decl) -> p)) string_append(&decl, " ");};
  string_append (&decl, qualifier_string (type_quals));
       }
     break;
   }
        case 'G':
   (*mangled)++;
   break;
 case 'C':
 case 'V':
 case 'u':
   if ((work -> options & (1 << 1)))
     {
       if (!((&decl) -> b == (&decl) -> p))
  string_prepend (&decl, " ");
       string_prepend (&decl, demangle_qualifier (*mangled));
     }
   (*mangled)++;
   break;
 default:
   done = 1;
   break;
 }
    }
  if (success) switch (*mangled)
    {
    case 'Q':
    case 'K':
      {
        success = demangle_qualified (work, mangled, result, 0, 1);
        break;
      }
    case 'B':
      (*mangled)++;
      if (!get_count (mangled, &n) || n >= work -> numb)
 success = 0;
      else
 string_append (result, work->btypevec[n]);
      break;
    case 'X':
    case 'Y':
      {
 int idx;
 (*mangled)++;
 idx = consume_count_with_underscores (mangled);
 if (idx == -1
     || (work->tmpl_argvec && idx >= work->ntmpl_args)
     || consume_count_with_underscores (mangled) == -1)
   {
     success = 0;
     break;
   }
 if (work->tmpl_argvec)
   string_append (result, work->tmpl_argvec[idx]);
 else
   string_append_template_idx (result, idx);
 success = 1;
      }
    break;
    default:
      success = demangle_fund_type (work, mangled, result);
      if (tk == tk_none)
 tk = (type_kind_t) success;
      break;
    }
  if (success)
    {
      if (!((&decl) -> b == (&decl) -> p))
 {
   string_append (result, " ");
   string_appends (result, &decl);
 }
    }
  else
    string_delete (result);
  string_delete (&decl);
  if (success)
    return (int) ((tk == tk_none) ? tk_integral : tk);
  else
    return 0;
}
static int
demangle_fund_type (work, mangled, result)
     struct work_stuff *work;
     const char *mangled;
     string *result;
{
  int done = 0;
  int success = 1;
  char buf[10];
  unsigned int dec = 0;
  type_kind_t tk = tk_integral;
  while (!done)
    {
      switch (*mangled)
 {
 case 'C':
 case 'V':
 case 'u':
   if ((work -> options & (1 << 1)))
     {
              if (!((result) -> b == (result) -> p))
                string_prepend (result, " ");
       string_prepend (result, demangle_qualifier (*mangled));
     }
   (*mangled)++;
   break;
 case 'U':
   (*mangled)++;
   {if (!((result) -> b == (result) -> p)) string_append(result, " ");};
   string_append (result, "unsigned");
   break;
 case 'S':
   (*mangled)++;
   {if (!((result) -> b == (result) -> p)) string_append(result, " ");};
   string_append (result, "signed");
   break;
 case 'J':
   (*mangled)++;
   {if (!((result) -> b == (result) -> p)) string_append(result, " ");};
   string_append (result, "__complex");
   break;
 default:
   done = 1;
   break;
 }
    }
  switch (*mangled)
    {
    case 0:
    case '_':
      break;
    case 'v':
      (*mangled)++;
      {if (!((result) -> b == (result) -> p)) string_append(result, " ");};
      string_append (result, "void");
      break;
    case 'x':
      (*mangled)++;
      {if (!((result) -> b == (result) -> p)) string_append(result, " ");};
      string_append (result, "long long");
      break;
    case 'l':
      (*mangled)++;
      {if (!((result) -> b == (result) -> p)) string_append(result, " ");};
      string_append (result, "long");
      break;
    case 'i':
      (*mangled)++;
      {if (!((result) -> b == (result) -> p)) string_append(result, " ");};
      string_append (result, "int");
      break;
    case 's':
      (*mangled)++;
      {if (!((result) -> b == (result) -> p)) string_append(result, " ");};
      string_append (result, "short");
      break;
    case 'b':
      (*mangled)++;
      {if (!((result) -> b == (result) -> p)) string_append(result, " ");};
      string_append (result, "bool");
      tk = tk_bool;
      break;
    case 'c':
      (*mangled)++;
      {if (!((result) -> b == (result) -> p)) string_append(result, " ");};
      string_append (result, "char");
      tk = tk_char;
      break;
    case 'w':
      (*mangled)++;
      {if (!((result) -> b == (result) -> p)) string_append(result, " ");};
      string_append (result, "wchar_t");
      tk = tk_char;
      break;
    case 'r':
      (*mangled)++;
      {if (!((result) -> b == (result) -> p)) string_append(result, " ");};
      string_append (result, "long double");
      tk = tk_real;
      break;
    case 'd':
      (*mangled)++;
      {if (!((result) -> b == (result) -> p)) string_append(result, " ");};
      string_append (result, "double");
      tk = tk_real;
      break;
    case 'f':
      (*mangled)++;
      {if (!((result) -> b == (result) -> p)) string_append(result, " ");};
      string_append (result, "float");
      tk = tk_real;
      break;
    case 'G':
      (*mangled)++;
      if (!(_sch_istable[((unsigned char)*mangled) & 0xff] & (unsigned short)(_sch_isdigit)))
 {
   success = 0;
   break;
 }
    case 'I':
      (*mangled)++;
      if (*mangled == '_')
 {
   int i;
   (*mangled)++;
   for (i = 0;
        i < (long) sizeof (buf) - 1 && *mangled && *mangled != '_';
        (*mangled)++, i++)
     buf[i] = *mangled;
   if (*mangled != '_')
     {
       success = 0;
       break;
     }
   buf[i] = 0;
   (*mangled)++;
 }
      else
 {
   strncpy (buf, *mangled, 2);
   buf[2] = 0;
   *mangled += (((strlen (*mangled)) < (2)) ? (strlen (*mangled)) : (2));
 }
      sscanf (buf, "%x", &dec);
      sprintf (buf, "int%u_t", dec);
      {if (!((result) -> b == (result) -> p)) string_append(result, " ");};
      string_append (result, buf);
      break;
    case '0':
    case '1':
    case '2':
    case '3':
    case '4':
    case '5':
    case '6':
    case '7':
    case '8':
    case '9':
      {
        int bindex = register_Btype (work);
        string btype;
        string_init (&btype);
        if (demangle_class_name (work, mangled, &btype)) {
          remember_Btype (work, btype.b, ( (((&btype) -> b == (&btype) -> p))?0:((&btype)->p - (&btype)->b)), bindex);
          {if (!((result) -> b == (result) -> p)) string_append(result, " ");};
          string_appends (result, &btype);
        }
        else
          success = 0;
        string_delete (&btype);
        break;
      }
    case 't':
      {
        string btype;
        string_init (&btype);
        success = demangle_template (work, mangled, &btype, 0, 1, 1);
        string_appends (result, &btype);
        string_delete (&btype);
        break;
      }
    default:
      success = 0;
      break;
    }
  return success ? ((int) tk) : 0;
}
static int
do_hpacc_template_const_value (work, mangled, result)
     struct work_stuff *work ;
     const char *mangled;
     string *result;
{
  int unsigned_const;
  if (*mangled != 'U' && *mangled != 'S')
    return 0;
  unsigned_const = (*mangled == 'U');
  (*mangled)++;
  switch (*mangled)
    {
      case 'N':
        string_append (result, "-");
      case 'P':
        (*mangled)++;
        break;
      case 'M':
        string_append (result, "-2147483648");
        (*mangled)++;
        return 1;
      default:
        return 0;
    }
  if (!((_sch_istable[((unsigned char)*mangled) & 0xff] & (unsigned short)(_sch_isdigit))))
    return 0;
  while ((_sch_istable[((unsigned char)*mangled) & 0xff] & (unsigned short)(_sch_isdigit)))
    {
      char_str[0] = *mangled;
      string_append (result, char_str);
      (*mangled)++;
    }
  if (unsigned_const)
    string_append (result, "U");
  return 1;
}
static int
do_hpacc_template_literal (work, mangled, result)
     struct work_stuff *work;
     const char *mangled;
     string *result;
{
  int literal_len = 0;
  char * recurse;
  char * recurse_dem;
  if (*mangled != 'A')
    return 0;
  (*mangled)++;
  literal_len = consume_count (mangled);
  if (literal_len <= 0)
    return 0;
  string_append (result, "&");
  recurse = (char *) xmalloc (literal_len + 1);
  memcpy (recurse, *mangled, literal_len);
  recurse[literal_len] = 0;
  recurse_dem = cplus_demangle (recurse, work->options);
  if (recurse_dem)
    {
      string_append (result, recurse_dem);
      free (recurse_dem);
    }
  else
    {
      string_appendn (result, *mangled, literal_len);
    }
  (*mangled) += literal_len;
  free (recurse);
  return 1;
}
static int
snarf_numeric_literal (args, arg)
     const char * args;
     string * arg;
{
  if (*args == '-')
    {
      char_str[0] = '-';
      string_append (arg, char_str);
      (*args)++;
    }
  else if (*args == '+')
    (*args)++;
  if (!(_sch_istable[((unsigned char)*args) & 0xff] & (unsigned short)(_sch_isdigit)))
    return 0;
  while ((_sch_istable[((unsigned char)*args) & 0xff] & (unsigned short)(_sch_isdigit)))
    {
      char_str[0] = *args;
      string_append (arg, char_str);
      (*args)++;
    }
  return 1;
}
static int
do_arg (work, mangled, result)
     struct work_stuff *work;
     const char *mangled;
     string *result;
{
  const char *start = *mangled;
  string_init (result);
  if (work->nrepeats > 0)
    {
      --work->nrepeats;
      if (work->previous_argument == 0)
 return 0;
      string_appends (result, work->previous_argument);
      return 1;
    }
  if (*mangled == 'n')
    {
      (*mangled)++;
      work->nrepeats = consume_count(mangled);
      if (work->nrepeats <= 0)
 return 0;
      if (work->nrepeats > 9)
 {
   if (*mangled != '_')
     return 0;
   else
     (*mangled)++;
 }
      return do_arg (work, mangled, result);
    }
  if (work->previous_argument)
    string_delete (work->previous_argument);
  else
    work->previous_argument = (string*) xmalloc (sizeof (string));
  if (!do_type (work, mangled, work->previous_argument))
    return 0;
  string_appends (result, work->previous_argument);
  remember_type (work, start, *mangled - start);
  return 1;
}
static void
remember_type (work, start, len)
     struct work_stuff *work;
     const char *start;
     int len;
{
  char *tem;
  if (work->forgetting_types)
    return;
  if (work -> ntypes >= work -> typevec_size)
    {
      if (work -> typevec_size == 0)
 {
   work -> typevec_size = 3;
   work -> typevec
     = (char *) xmalloc (sizeof (char *) * work -> typevec_size);
 }
      else
 {
   work -> typevec_size *= 2;
   work -> typevec
     = (char *) xrealloc ((char *)work -> typevec,
      sizeof (char *) * work -> typevec_size);
 }
    }
  tem = xmalloc (len + 1);
  memcpy (tem, start, len);
  tem[len] = 0;
  work -> typevec[work -> ntypes++] = tem;
}
static void
remember_Ktype (work, start, len)
     struct work_stuff *work;
     const char *start;
     int len;
{
  char *tem;
  if (work -> numk >= work -> ksize)
    {
      if (work -> ksize == 0)
 {
   work -> ksize = 5;
   work -> ktypevec
     = (char *) xmalloc (sizeof (char *) * work -> ksize);
 }
      else
 {
   work -> ksize *= 2;
   work -> ktypevec
     = (char *) xrealloc ((char *)work -> ktypevec,
      sizeof (char *) * work -> ksize);
 }
    }
  tem = xmalloc (len + 1);
  memcpy (tem, start, len);
  tem[len] = 0;
  work -> ktypevec[work -> numk++] = tem;
}
static int
register_Btype (work)
     struct work_stuff *work;
{
  int ret;
  if (work -> numb >= work -> bsize)
    {
      if (work -> bsize == 0)
 {
   work -> bsize = 5;
   work -> btypevec
     = (char *) xmalloc (sizeof (char *) * work -> bsize);
 }
      else
 {
   work -> bsize *= 2;
   work -> btypevec
     = (char *) xrealloc ((char *)work -> btypevec,
      sizeof (char *) * work -> bsize);
 }
    }
  ret = work -> numb++;
  work -> btypevec[ret] = ((void *)0);
  return(ret);
}
static void
remember_Btype (work, start, len, index)
     struct work_stuff *work;
     const char *start;
     int len, index;
{
  char *tem;
  tem = xmalloc (len + 1);
  memcpy (tem, start, len);
  tem[len] = 0;
  work -> btypevec[index] = tem;
}
static void
forget_B_and_K_types (work)
     struct work_stuff *work;
{
  int i;
  while (work -> numk > 0)
    {
      i = --(work -> numk);
      if (work -> ktypevec[i] != ((void *)0))
 {
   free (work -> ktypevec[i]);
   work -> ktypevec[i] = ((void *)0);
 }
    }
  while (work -> numb > 0)
    {
      i = --(work -> numb);
      if (work -> btypevec[i] != ((void *)0))
 {
   free (work -> btypevec[i]);
   work -> btypevec[i] = ((void *)0);
 }
    }
}
static void
forget_types (work)
     struct work_stuff *work;
{
  int i;
  while (work -> ntypes > 0)
    {
      i = --(work -> ntypes);
      if (work -> typevec[i] != ((void *)0))
 {
   free (work -> typevec[i]);
   work -> typevec[i] = ((void *)0);
 }
    }
}
static int
demangle_args (work, mangled, declp)
     struct work_stuff *work;
     const char *mangled;
     string *declp;
{
  string arg;
  int need_comma = 0;
  int r;
  int t;
  const char *tem;
  char temptype;
  if ((work -> options & (1 << 0)))
    {
      string_append (declp, "(");
      if (*mangled == 0)
 {
   string_append (declp, "void");
 }
    }
  while ((*mangled != '_' && *mangled != 0 && *mangled != 'e')
  || work->nrepeats > 0)
    {
      if ((*mangled == 'N') || (*mangled == 'T'))
 {
   temptype = *(*mangled)++;
   if (temptype == 'N')
     {
       if (!get_count (mangled, &r))
  {
    return (0);
  }
     }
   else
     {
       r = 1;
     }
          if (((((int) work->options) & (1 << 12)) || (((int) work->options) & (1 << 11)) || (((int) work->options) & (1 << 13))) && work -> ntypes >= 10)
            {
              if ((t = consume_count(mangled)) <= 0)
                {
                  return (0);
                }
            }
          else
     {
       if (!get_count (mangled, &t))
      {
           return (0);
      }
     }
   if ((((int) work->options) & (1 << 10)) || (((int) work->options) & (1 << 11)) || (((int) work->options) & (1 << 12)) || (((int) work->options) & (1 << 13)))
     {
       t--;
     }
   if ((t < 0) || (t >= work -> ntypes))
     {
       return (0);
     }
   while (work->nrepeats > 0 || --r >= 0)
     {
       tem = work -> typevec[t];
       if (need_comma && (work -> options & (1 << 0)))
  {
    string_append (declp, ", ");
  }
       if (!do_arg (work, &tem, &arg))
  {
    return (0);
  }
       if ((work -> options & (1 << 0)))
  {
    string_appends (declp, &arg);
  }
       string_delete (&arg);
       need_comma = 1;
     }
 }
      else
 {
   if (need_comma && (work -> options & (1 << 0)))
     string_append (declp, ", ");
   if (!do_arg (work, mangled, &arg))
     return (0);
   if ((work -> options & (1 << 0)))
     string_appends (declp, &arg);
   string_delete (&arg);
   need_comma = 1;
 }
    }
  if (*mangled == 'e')
    {
      (*mangled)++;
      if ((work -> options & (1 << 0)))
 {
   if (need_comma)
     {
       string_append (declp, ",");
     }
   string_append (declp, "...");
 }
    }
  if ((work -> options & (1 << 0)))
    {
      string_append (declp, ")");
    }
  return (1);
}
static int
demangle_nested_args (work, mangled, declp)
     struct work_stuff *work;
     const char *mangled;
     string *declp;
{
  string* saved_previous_argument;
  int result;
  int saved_nrepeats;
  ++work->forgetting_types;
  saved_previous_argument = work->previous_argument;
  saved_nrepeats = work->nrepeats;
  work->previous_argument = 0;
  work->nrepeats = 0;
  result = demangle_args (work, mangled, declp);
  if (work->previous_argument)
    {
      string_delete (work->previous_argument);
      free ((char *) work->previous_argument);
    }
  work->previous_argument = saved_previous_argument;
  --work->forgetting_types;
  work->nrepeats = saved_nrepeats;
  return result;
}
static void
demangle_function_name (work, mangled, declp, scan)
     struct work_stuff *work;
     const char *mangled;
     string *declp;
     const char *scan;
{
  size_t i;
  string type;
  const char *tem;
  string_appendn (declp, (*mangled), scan - (*mangled));
  string_need (declp, 1);
  *(declp -> p) = 0;
  (*mangled) = scan + 2;
  if ((((int) work->options) & (1 << 12)) && (*mangled == 'X'))
    {
      demangle_arm_hp_template (work, mangled, 0, declp);
    }
  if ((((int) work->options) & (1 << 10)) || (((int) work->options) & (1 << 11)) || (((int) work->options) & (1 << 12)) || (((int) work->options) & (1 << 13)))
    {
      if (strcmp (declp -> b, "__ct") == 0)
 {
   work -> constructor += 1;
   string_clear (declp);
   return;
 }
      else if (strcmp (declp -> b, "__dt") == 0)
 {
   work -> destructor += 1;
   string_clear (declp);
   return;
 }
    }
  if (declp->p - declp->b >= 3
      && declp->b[0] == 'o'
      && declp->b[1] == 'p'
      && strchr (cplus_markers, declp->b[2]) != ((void *)0))
    {
      if (declp->p - declp->b >= 10
   && memcmp (declp->b + 3, "assign_", 7) == 0)
 {
   for (i = 0; i < (sizeof (optable) / sizeof ((optable)[0])); i++)
     {
       int len = declp->p - declp->b - 10;
       if ((int) strlen (optable[i].in) == len
    && memcmp (optable[i].in, declp->b + 10, len) == 0)
  {
    string_clear (declp);
    string_append (declp, "operator");
    string_append (declp, optable[i].out);
    string_append (declp, "=");
    break;
  }
     }
 }
      else
 {
   for (i = 0; i < (sizeof (optable) / sizeof ((optable)[0])); i++)
     {
       int len = declp->p - declp->b - 3;
       if ((int) strlen (optable[i].in) == len
    && memcmp (optable[i].in, declp->b + 3, len) == 0)
  {
    string_clear (declp);
    string_append (declp, "operator");
    string_append (declp, optable[i].out);
    break;
  }
     }
 }
    }
  else if (declp->p - declp->b >= 5 && memcmp (declp->b, "type", 4) == 0
    && strchr (cplus_markers, declp->b[4]) != ((void *)0))
    {
      tem = declp->b + 5;
      if (do_type (work, &tem, &type))
 {
   string_clear (declp);
   string_append (declp, "operator ");
   string_appends (declp, &type);
   string_delete (&type);
 }
    }
  else if (declp->b[0] == '_' && declp->b[1] == '_'
    && declp->b[2] == 'o' && declp->b[3] == 'p')
    {
      tem = declp->b + 4;
      if (do_type (work, &tem, &type))
 {
   string_clear (declp);
   string_append (declp, "operator ");
   string_appends (declp, &type);
   string_delete (&type);
 }
    }
  else if (declp->b[0] == '_' && declp->b[1] == '_'
    && (_sch_istable[((unsigned char)declp->b[2]) & 0xff] & (unsigned short)(_sch_islower))
    && (_sch_istable[((unsigned char)declp->b[3]) & 0xff] & (unsigned short)(_sch_islower)))
    {
      if (declp->b[4] == 0)
 {
   for (i = 0; i < (sizeof (optable) / sizeof ((optable)[0])); i++)
     {
       if (strlen (optable[i].in) == 2
    && memcmp (optable[i].in, declp->b + 2, 2) == 0)
  {
    string_clear (declp);
    string_append (declp, "operator");
    string_append (declp, optable[i].out);
    break;
  }
     }
 }
      else
 {
   if (declp->b[2] == 'a' && declp->b[5] == 0)
     {
       for (i = 0; i < (sizeof (optable) / sizeof ((optable)[0])); i++)
  {
    if (strlen (optable[i].in) == 3
        && memcmp (optable[i].in, declp->b + 2, 3) == 0)
      {
        string_clear (declp);
        string_append (declp, "operator");
        string_append (declp, optable[i].out);
        break;
      }
  }
     }
 }
    }
}
static void
string_need (s, n)
     string *s;
     int n;
{
  int tem;
  if (s->b == ((void *)0))
    {
      if (n < 32)
 {
   n = 32;
 }
      s->p = s->b = xmalloc (n);
      s->e = s->b + n;
    }
  else if (s->e - s->p < n)
    {
      tem = s->p - s->b;
      n += tem;
      n *= 2;
      s->b = xrealloc (s->b, n);
      s->p = s->b + tem;
      s->e = s->b + n;
    }
}
static void
string_delete (s)
     string *s;
{
  if (s->b != ((void *)0))
    {
      free (s->b);
      s->b = s->e = s->p = ((void *)0);
    }
}
static void
string_init (s)
     string *s;
{
  s->b = s->p = s->e = ((void *)0);
}
static void
string_clear (s)
     string *s;
{
  s->p = s->b;
}
static void
string_append (p, s)
     string *p;
     const char *s;
{
  int n;
  if (s == ((void *)0) || *s == 0)
    return;
  n = strlen (s);
  string_need (p, n);
  memcpy (p->p, s, n);
  p->p += n;
}
static void
string_appends (p, s)
     string *p, *s;
{
  int n;
  if (s->b != s->p)
    {
      n = s->p - s->b;
      string_need (p, n);
      memcpy (p->p, s->b, n);
      p->p += n;
    }
}
static void
string_appendn (p, s, n)
     string *p;
     const char *s;
     int n;
{
  if (n != 0)
    {
      string_need (p, n);
      memcpy (p->p, s, n);
      p->p += n;
    }
}
static void
string_prepend (p, s)
     string *p;
     const char *s;
{
  if (s != ((void *)0) && *s != 0)
    {
      string_prependn (p, s, strlen (s));
    }
}
static void
string_prepends (p, s)
     string *p, *s;
{
  if (s->b != s->p)
    {
      string_prependn (p, s->b, s->p - s->b);
    }
}
static void
string_prependn (p, s, n)
     string *p;
     const char *s;
     int n;
{
  char *q;
  if (n != 0)
    {
      string_need (p, n);
      for (q = p->p - 1; q >= p->b; q--)
 {
   q[n] = q[0];
 }
      memcpy (p->b, s, n);
      p->p += n;
    }
}
static void
string_append_template_idx (s, idx)
     string *s;
     int idx;
{
  char buf[32 + 1 ];
  sprintf(buf, "T%d", idx);
  string_append (s, buf);
}
struct demangle_operator_info
{
  const char *code;
  const char *name;
  int len;
  int args;
};
enum d_builtin_type_print
{
  D_PRINT_DEFAULT,
  D_PRINT_INT,
  D_PRINT_UNSIGNED,
  D_PRINT_LONG,
  D_PRINT_UNSIGNED_LONG,
  D_PRINT_LONG_LONG,
  D_PRINT_UNSIGNED_LONG_LONG,
  D_PRINT_BOOL,
  D_PRINT_FLOAT,
  D_PRINT_VOID
};
struct demangle_builtin_type_info
{
  const char *name;
  int len;
  const char *java_name;
  int java_len;
  enum d_builtin_type_print print;
};
struct d_info
{
  const char *s;
  const char *send;
  int options;
  const char *n;
  struct demangle_component *comps;
  int next_comp;
  int num_comps;
  struct demangle_component *subs;
  int next_sub;
  int num_subs;
  int did_subs;
  struct demangle_component *last_name;
  int expansion;
};
extern
const struct demangle_operator_info cplus_demangle_operators[];
extern
const struct demangle_builtin_type_info
cplus_demangle_builtin_types[(26)];
extern
struct demangle_component *
cplus_demangle_mangled_name (struct d_info *, int);
extern
struct demangle_component *
cplus_demangle_type (struct d_info *);
extern void
cplus_demangle_init_info (const char *, int, size_t, struct d_info *);
struct d_standard_sub_info
{
  char code;
  const char *simple_expansion;
  int simple_len;
  const char *full_expansion;
  int full_len;
  const char *set_last_name;
  int set_last_name_len;
};
struct d_print_template
{
  struct d_print_template *next;
  const struct demangle_component *template;
};
struct d_print_mod
{
  struct d_print_mod *next;
  const struct demangle_component *mod;
  int printed;
  struct d_print_template *templates;
};
struct d_print_info
{
  int options;
  char *buf;
  size_t len;
  size_t alc;
  struct d_print_template *templates;
  struct d_print_mod *modifiers;
  int allocation_failure;
};
static struct demangle_component *
d_make_empty (struct d_info *);
static struct demangle_component *
d_make_comp (struct d_info *, enum demangle_component_type, struct demangle_component *, struct demangle_component *)
                                    ;
static struct demangle_component *
d_make_name (struct d_info *, const char *, int);
static struct demangle_component *
d_make_builtin_type (struct d_info *, const struct demangle_builtin_type_info *)
                                                   ;
static struct demangle_component *
d_make_operator (struct d_info *, const struct demangle_operator_info *)
                                           ;
static struct demangle_component *
d_make_extended_operator (struct d_info *, int, struct demangle_component *)
                                   ;
static struct demangle_component *
d_make_ctor (struct d_info *, enum gnu_v3_ctor_kinds, struct demangle_component *)
                                    ;
static struct demangle_component *
d_make_dtor (struct d_info *, enum gnu_v3_dtor_kinds, struct demangle_component *)
                                    ;
static struct demangle_component *
d_make_template_param (struct d_info *, long);
static struct demangle_component *
d_make_sub (struct d_info *, const char *, int);
static int
has_return_type (struct demangle_component *);
static int
is_ctor_dtor_or_conversion (struct demangle_component *);
static struct demangle_component *
d_encoding (struct d_info *, int);
static struct demangle_component *
d_name (struct d_info *);
static struct demangle_component *
d_nested_name (struct d_info *);
static struct demangle_component *
d_prefix (struct d_info *);
static struct demangle_component *
d_unqualified_name (struct d_info *);
static struct demangle_component *
d_source_name (struct d_info *);
static long
d_number (struct d_info *);
static struct demangle_component *
d_identifier (struct d_info *, int);
static struct demangle_component *
d_operator_name (struct d_info *);
static struct demangle_component *
d_special_name (struct d_info *);
static int
d_call_offset (struct d_info *, int);
static struct demangle_component *
d_ctor_dtor_name (struct d_info *);
static struct demangle_component *
d_cv_qualifiers (struct d_info *, struct demangle_component *, int);
static struct demangle_component *
d_function_type (struct d_info *);
static struct demangle_component *
d_bare_function_type (struct d_info *, int);
static struct demangle_component *
d_class_enum_type (struct d_info *);
static struct demangle_component *
d_array_type (struct d_info *);
static struct demangle_component *
d_pointer_to_member_type (struct d_info *);
static struct demangle_component *
d_template_param (struct d_info *);
static struct demangle_component *
d_template_args (struct d_info *);
static struct demangle_component *
d_template_arg (struct d_info *);
static struct demangle_component *
d_expression (struct d_info *);
static struct demangle_component *
d_expr_primary (struct d_info *);
static struct demangle_component *
d_local_name (struct d_info *);
static int
d_discriminator (struct d_info *);
static int
d_add_substitution (struct d_info *, struct demangle_component *);
static struct demangle_component *
d_substitution (struct d_info *, int);
static void
d_print_resize (struct d_print_info *, size_t);
static void
d_print_append_char (struct d_print_info *, int);
static void
d_print_append_buffer (struct d_print_info *, const char *, size_t);
static void
d_print_error (struct d_print_info *);
static void
d_print_comp (struct d_print_info *, const struct demangle_component *)
                                           ;
static void
d_print_java_identifier (struct d_print_info *, const char *, int);
static void
d_print_mod_list (struct d_print_info *, struct d_print_mod *, int);
static void
d_print_mod (struct d_print_info *, const struct demangle_component *)
                                          ;
static void
d_print_function_type (struct d_print_info *, const struct demangle_component *, struct d_print_mod *)
                                ;
static void
d_print_array_type (struct d_print_info *, const struct demangle_component *, struct d_print_mod *)
                             ;
static void
d_print_expr_op (struct d_print_info *, const struct demangle_component *)
                                       ;
static void
d_print_cast (struct d_print_info *, const struct demangle_component *)
                                           ;
static char *
d_demangle (const char *, int, size_t *);
int
cplus_demangle_fill_name (p, s, len)
     struct demangle_component *p;
     const char *s;
     int len;
{
  if (p == ((void *)0) || s == ((void *)0) || len == 0)
    return 0;
  p->type = DEMANGLE_COMPONENT_NAME;
  p->u.s_name.s = s;
  p->u.s_name.len = len;
  return 1;
}
int
cplus_demangle_fill_extended_operator (p, args, name)
     struct demangle_component *p;
     int args;
     struct demangle_component *name;
{
  if (p == ((void *)0) || args < 0 || name == ((void *)0))
    return 0;
  p->type = DEMANGLE_COMPONENT_EXTENDED_OPERATOR;
  p->u.s_extended_operator.args = args;
  p->u.s_extended_operator.name = name;
  return 1;
}
int
cplus_demangle_fill_ctor (p, kind, name)
     struct demangle_component *p;
     enum gnu_v3_ctor_kinds kind;
     struct demangle_component *name;
{
  if (p == ((void *)0)
      || name == ((void *)0)
      || (kind < gnu_v3_complete_object_ctor
   && kind > gnu_v3_complete_object_allocating_ctor))
    return 0;
  p->type = DEMANGLE_COMPONENT_CTOR;
  p->u.s_ctor.kind = kind;
  p->u.s_ctor.name = name;
  return 1;
}
int
cplus_demangle_fill_dtor (p, kind, name)
     struct demangle_component *p;
     enum gnu_v3_dtor_kinds kind;
     struct demangle_component *name;
{
  if (p == ((void *)0)
      || name == ((void *)0)
      || (kind < gnu_v3_deleting_dtor
   && kind > gnu_v3_base_object_dtor))
    return 0;
  p->type = DEMANGLE_COMPONENT_DTOR;
  p->u.s_dtor.kind = kind;
  p->u.s_dtor.name = name;
  return 1;
}
static struct demangle_component *
d_make_empty (di)
     struct d_info *di;
{
  struct demangle_component *p;
  if (di->next_comp >= di->num_comps)
    return ((void *)0);
  p = &di->comps[di->next_comp];
  ++di->next_comp;
  return p;
}
static struct demangle_component *
d_make_comp (di, type, left, right)
     struct d_info *di;
     enum demangle_component_type type;
     struct demangle_component *left;
     struct demangle_component *right;
{
  struct demangle_component *p;
  switch (type)
    {
    case DEMANGLE_COMPONENT_QUAL_NAME:
    case DEMANGLE_COMPONENT_LOCAL_NAME:
    case DEMANGLE_COMPONENT_TYPED_NAME:
    case DEMANGLE_COMPONENT_TEMPLATE:
    case DEMANGLE_COMPONENT_CONSTRUCTION_VTABLE:
    case DEMANGLE_COMPONENT_VENDOR_TYPE_QUAL:
    case DEMANGLE_COMPONENT_PTRMEM_TYPE:
    case DEMANGLE_COMPONENT_UNARY:
    case DEMANGLE_COMPONENT_BINARY:
    case DEMANGLE_COMPONENT_BINARY_ARGS:
    case DEMANGLE_COMPONENT_TRINARY:
    case DEMANGLE_COMPONENT_TRINARY_ARG1:
    case DEMANGLE_COMPONENT_TRINARY_ARG2:
    case DEMANGLE_COMPONENT_LITERAL:
    case DEMANGLE_COMPONENT_LITERAL_NEG:
      if (left == ((void *)0) || right == ((void *)0))
 return ((void *)0);
      break;
    case DEMANGLE_COMPONENT_VTABLE:
    case DEMANGLE_COMPONENT_VTT:
    case DEMANGLE_COMPONENT_TYPEINFO:
    case DEMANGLE_COMPONENT_TYPEINFO_NAME:
    case DEMANGLE_COMPONENT_TYPEINFO_FN:
    case DEMANGLE_COMPONENT_THUNK:
    case DEMANGLE_COMPONENT_VIRTUAL_THUNK:
    case DEMANGLE_COMPONENT_COVARIANT_THUNK:
    case DEMANGLE_COMPONENT_JAVA_CLASS:
    case DEMANGLE_COMPONENT_GUARD:
    case DEMANGLE_COMPONENT_REFTEMP:
    case DEMANGLE_COMPONENT_POINTER:
    case DEMANGLE_COMPONENT_REFERENCE:
    case DEMANGLE_COMPONENT_COMPLEX:
    case DEMANGLE_COMPONENT_IMAGINARY:
    case DEMANGLE_COMPONENT_VENDOR_TYPE:
    case DEMANGLE_COMPONENT_ARGLIST:
    case DEMANGLE_COMPONENT_TEMPLATE_ARGLIST:
    case DEMANGLE_COMPONENT_CAST:
      if (left == ((void *)0))
 return ((void *)0);
      break;
    case DEMANGLE_COMPONENT_ARRAY_TYPE:
      if (right == ((void *)0))
 return ((void *)0);
      break;
    case DEMANGLE_COMPONENT_FUNCTION_TYPE:
    case DEMANGLE_COMPONENT_RESTRICT:
    case DEMANGLE_COMPONENT_VOLATILE:
    case DEMANGLE_COMPONENT_CONST:
    case DEMANGLE_COMPONENT_RESTRICT_THIS:
    case DEMANGLE_COMPONENT_VOLATILE_THIS:
    case DEMANGLE_COMPONENT_CONST_THIS:
      break;
    default:
      return ((void *)0);
    }
  p = d_make_empty (di);
  if (p != ((void *)0))
    {
      p->type = type;
      p->u.s_binary.left = left;
      p->u.s_binary.right = right;
    }
  return p;
}
static struct demangle_component *
d_make_name (di, s, len)
     struct d_info *di;
     const char *s;
     int len;
{
  struct demangle_component *p;
  p = d_make_empty (di);
  if (! cplus_demangle_fill_name (p, s, len))
    return ((void *)0);
  return p;
}
static struct demangle_component *
d_make_builtin_type (di, type)
     struct d_info *di;
     const struct demangle_builtin_type_info *type;
{
  struct demangle_component *p;
  if (type == ((void *)0))
    return ((void *)0);
  p = d_make_empty (di);
  if (p != ((void *)0))
    {
      p->type = DEMANGLE_COMPONENT_BUILTIN_TYPE;
      p->u.s_builtin.type = type;
    }
  return p;
}
static struct demangle_component *
d_make_operator (di, op)
     struct d_info *di;
     const struct demangle_operator_info *op;
{
  struct demangle_component *p;
  p = d_make_empty (di);
  if (p != ((void *)0))
    {
      p->type = DEMANGLE_COMPONENT_OPERATOR;
      p->u.s_operator.op = op;
    }
  return p;
}
static struct demangle_component *
d_make_extended_operator (di, args, name)
     struct d_info *di;
     int args;
     struct demangle_component *name;
{
  struct demangle_component *p;
  p = d_make_empty (di);
  if (! cplus_demangle_fill_extended_operator (p, args, name))
    return ((void *)0);
  return p;
}
static struct demangle_component *
d_make_ctor (di, kind, name)
     struct d_info *di;
     enum gnu_v3_ctor_kinds kind;
     struct demangle_component *name;
{
  struct demangle_component *p;
  p = d_make_empty (di);
  if (! cplus_demangle_fill_ctor (p, kind, name))
    return ((void *)0);
  return p;
}
static struct demangle_component *
d_make_dtor (di, kind, name)
     struct d_info *di;
     enum gnu_v3_dtor_kinds kind;
     struct demangle_component *name;
{
  struct demangle_component *p;
  p = d_make_empty (di);
  if (! cplus_demangle_fill_dtor (p, kind, name))
    return ((void *)0);
  return p;
}
static struct demangle_component *
d_make_template_param (di, i)
     struct d_info *di;
     long i;
{
  struct demangle_component *p;
  p = d_make_empty (di);
  if (p != ((void *)0))
    {
      p->type = DEMANGLE_COMPONENT_TEMPLATE_PARAM;
      p->u.s_number.number = i;
    }
  return p;
}
static struct demangle_component *
d_make_sub (di, name, len)
     struct d_info *di;
     const char *name;
     int len;
{
  struct demangle_component *p;
  p = d_make_empty (di);
  if (p != ((void *)0))
    {
      p->type = DEMANGLE_COMPONENT_SUB_STD;
      p->u.s_string.string1 = name;
      p->u.s_string.len = len;
    }
  return p;
}
struct demangle_component *
cplus_demangle_mangled_name (di, top_level)
     struct d_info *di;
     int top_level;
{
  if ((*((di)->n++)) != '_')
    return ((void *)0);
  if ((*((di)->n++)) != 'Z')
    return ((void *)0);
  return d_encoding (di, top_level);
}
static int
has_return_type (dc)
     struct demangle_component *dc;
{
  if (dc == ((void *)0))
    return 0;
  switch (dc->type)
    {
    default:
      return 0;
    case DEMANGLE_COMPONENT_TEMPLATE:
      return ! is_ctor_dtor_or_conversion (((dc)->u.s_binary.left));
    case DEMANGLE_COMPONENT_RESTRICT_THIS:
    case DEMANGLE_COMPONENT_VOLATILE_THIS:
    case DEMANGLE_COMPONENT_CONST_THIS:
      return has_return_type (((dc)->u.s_binary.left));
    }
}
static int
is_ctor_dtor_or_conversion (dc)
     struct demangle_component *dc;
{
  if (dc == ((void *)0))
    return 0;
  switch (dc->type)
    {
    default:
      return 0;
    case DEMANGLE_COMPONENT_QUAL_NAME:
    case DEMANGLE_COMPONENT_LOCAL_NAME:
      return is_ctor_dtor_or_conversion (((dc)->u.s_binary.right));
    case DEMANGLE_COMPONENT_CTOR:
    case DEMANGLE_COMPONENT_DTOR:
    case DEMANGLE_COMPONENT_CAST:
      return 1;
    }
}
static struct demangle_component *
d_encoding (di, top_level)
     struct d_info *di;
     int top_level;
{
  char peek = (*((di)->n));
  if (peek == 'G' || peek == 'T')
    return d_special_name (di);
  else
    {
      struct demangle_component *dc;
      dc = d_name (di);
      if (dc != ((void *)0) && top_level && (di->options & (1 << 0)) == 0)
 {
   while (dc->type == DEMANGLE_COMPONENT_RESTRICT_THIS
   || dc->type == DEMANGLE_COMPONENT_VOLATILE_THIS
   || dc->type == DEMANGLE_COMPONENT_CONST_THIS)
     dc = ((dc)->u.s_binary.left);
   if (dc->type == DEMANGLE_COMPONENT_LOCAL_NAME)
     {
       struct demangle_component *dcr;
       dcr = ((dc)->u.s_binary.right);
       while (dcr->type == DEMANGLE_COMPONENT_RESTRICT_THIS
       || dcr->type == DEMANGLE_COMPONENT_VOLATILE_THIS
       || dcr->type == DEMANGLE_COMPONENT_CONST_THIS)
  dcr = ((dcr)->u.s_binary.left);
       dc->u.s_binary.right = dcr;
     }
   return dc;
 }
      peek = (*((di)->n));
      if (peek == 0 || peek == 'E')
 return dc;
      return d_make_comp (di, DEMANGLE_COMPONENT_TYPED_NAME, dc,
     d_bare_function_type (di, has_return_type (dc)));
    }
}
static struct demangle_component *
d_name (di)
     struct d_info *di;
{
  char peek = (*((di)->n));
  struct demangle_component *dc;
  switch (peek)
    {
    case 'N':
      return d_nested_name (di);
    case 'Z':
      return d_local_name (di);
    case 'S':
      {
 int subst;
 if (((di)->n[1]) != 't')
   {
     dc = d_substitution (di, 0);
     subst = 1;
   }
 else
   {
     ((di)->n += (2));
     dc = d_make_comp (di, DEMANGLE_COMPONENT_QUAL_NAME,
         d_make_name (di, "std", 3),
         d_unqualified_name (di));
     di->expansion += 3;
     subst = 0;
   }
 if ((*((di)->n)) != 'I')
   {
   }
 else
   {
     if (! subst)
       {
  if (! d_add_substitution (di, dc))
    return ((void *)0);
       }
     dc = d_make_comp (di, DEMANGLE_COMPONENT_TEMPLATE, dc,
         d_template_args (di));
   }
 return dc;
      }
    default:
      dc = d_unqualified_name (di);
      if ((*((di)->n)) == 'I')
 {
   if (! d_add_substitution (di, dc))
     return ((void *)0);
   dc = d_make_comp (di, DEMANGLE_COMPONENT_TEMPLATE, dc,
       d_template_args (di));
 }
      return dc;
    }
}
static struct demangle_component *
d_nested_name (di)
     struct d_info *di;
{
  struct demangle_component *ret;
  struct demangle_component *pret;
  if ((*((di)->n++)) != 'N')
    return ((void *)0);
  pret = d_cv_qualifiers (di, &ret, 1);
  if (pret == ((void *)0))
    return ((void *)0);
  *pret = d_prefix (di);
  if (*pret == ((void *)0))
    return ((void *)0);
  if ((*((di)->n++)) != 'E')
    return ((void *)0);
  return ret;
}
static struct demangle_component *
d_prefix (di)
     struct d_info *di;
{
  struct demangle_component *ret = ((void *)0);
  while (1)
    {
      char peek;
      enum demangle_component_type comb_type;
      struct demangle_component *dc;
      peek = (*((di)->n));
      if (peek == 0)
 return ((void *)0);
      comb_type = DEMANGLE_COMPONENT_QUAL_NAME;
      if (((peek) >= '0' && (peek) <= '9')
   || ((peek) >= 'a' && (peek) <= 'z')
   || peek == 'C'
   || peek == 'D')
 dc = d_unqualified_name (di);
      else if (peek == 'S')
 dc = d_substitution (di, 1);
      else if (peek == 'I')
 {
   if (ret == ((void *)0))
     return ((void *)0);
   comb_type = DEMANGLE_COMPONENT_TEMPLATE;
   dc = d_template_args (di);
 }
      else if (peek == 'T')
 dc = d_template_param (di);
      else if (peek == 'E')
 return ret;
      else
 return ((void *)0);
      if (ret == ((void *)0))
 ret = dc;
      else
 ret = d_make_comp (di, comb_type, ret, dc);
      if (peek != 'S' && (*((di)->n)) != 'E')
 {
   if (! d_add_substitution (di, ret))
     return ((void *)0);
 }
    }
}
static struct demangle_component *
d_unqualified_name (di)
     struct d_info *di;
{
  char peek;
  peek = (*((di)->n));
  if (((peek) >= '0' && (peek) <= '9'))
    return d_source_name (di);
  else if (((peek) >= 'a' && (peek) <= 'z'))
    {
      struct demangle_component *ret;
      ret = d_operator_name (di);
      if (ret != ((void *)0) && ret->type == DEMANGLE_COMPONENT_OPERATOR)
 di->expansion += sizeof "operator" + ret->u.s_operator.op->len - 2;
      return ret;
    }
  else if (peek == 'C' || peek == 'D')
    return d_ctor_dtor_name (di);
  else
    return ((void *)0);
}
static struct demangle_component *
d_source_name (di)
     struct d_info *di;
{
  long len;
  struct demangle_component *ret;
  len = d_number (di);
  if (len <= 0)
    return ((void *)0);
  ret = d_identifier (di, len);
  di->last_name = ret;
  return ret;
}
static long
d_number (di)
     struct d_info *di;
{
  int negative;
  char peek;
  long ret;
  negative = 0;
  peek = (*((di)->n));
  if (peek == 'n')
    {
      negative = 1;
      ((di)->n += (1));
      peek = (*((di)->n));
    }
  ret = 0;
  while (1)
    {
      if (! ((peek) >= '0' && (peek) <= '9'))
 {
   if (negative)
     ret = - ret;
   return ret;
 }
      ret = ret * 10 + peek - '0';
      ((di)->n += (1));
      peek = (*((di)->n));
    }
}
static struct demangle_component *
d_identifier (di, len)
     struct d_info *di;
     int len;
{
  const char *name;
  name = ((di)->n);
  if (di->send - name < len)
    return ((void *)0);
  ((di)->n += (len));
  if ((di->options & (1 << 2)) != 0
      && (*((di)->n)) == '$')
    ((di)->n += (1));
  if (len >= (int) (sizeof ("_GLOBAL_") - 1) + 2
      && memcmp (name, "_GLOBAL_",
   (sizeof ("_GLOBAL_") - 1)) == 0)
    {
      const char *s;
      s = name + (sizeof ("_GLOBAL_") - 1);
      if ((*s == '.' || *s == '_' || *s == '$')
   && s[1] == 'N')
 {
   di->expansion -= len - sizeof "(anonymous namespace)";
   return d_make_name (di, "(anonymous namespace)",
         sizeof "(anonymous namespace)" - 1);
 }
    }
  return d_make_name (di, name, len);
}
const struct demangle_operator_info cplus_demangle_operators[] =
{
  { "aN", "&=", (sizeof "&=") - 1, 2 },
  { "aS", "=", (sizeof "=") - 1, 2 },
  { "aa", "&&", (sizeof "&&") - 1, 2 },
  { "ad", "&", (sizeof "&") - 1, 1 },
  { "an", "&", (sizeof "&") - 1, 2 },
  { "cl", "()", (sizeof "()") - 1, 0 },
  { "cm", ",", (sizeof ",") - 1, 2 },
  { "co", "~", (sizeof "~") - 1, 1 },
  { "dV", "/=", (sizeof "/=") - 1, 2 },
  { "da", "delete[]", (sizeof "delete[]") - 1, 1 },
  { "de", "*", (sizeof "*") - 1, 1 },
  { "dl", "delete", (sizeof "delete") - 1, 1 },
  { "dv", "/", (sizeof "/") - 1, 2 },
  { "eO", "^=", (sizeof "^=") - 1, 2 },
  { "eo", "^", (sizeof "^") - 1, 2 },
  { "eq", "==", (sizeof "==") - 1, 2 },
  { "ge", ">=", (sizeof ">=") - 1, 2 },
  { "gt", ">", (sizeof ">") - 1, 2 },
  { "ix", "[]", (sizeof "[]") - 1, 2 },
  { "lS", "<<=", (sizeof "<<=") - 1, 2 },
  { "le", "<=", (sizeof "<=") - 1, 2 },
  { "ls", "<<", (sizeof "<<") - 1, 2 },
  { "lt", "<", (sizeof "<") - 1, 2 },
  { "mI", "-=", (sizeof "-=") - 1, 2 },
  { "mL", "*=", (sizeof "*=") - 1, 2 },
  { "mi", "-", (sizeof "-") - 1, 2 },
  { "ml", "*", (sizeof "*") - 1, 2 },
  { "mm", "--", (sizeof "--") - 1, 1 },
  { "na", "new[]", (sizeof "new[]") - 1, 1 },
  { "ne", "!=", (sizeof "!=") - 1, 2 },
  { "ng", "-", (sizeof "-") - 1, 1 },
  { "nt", "!", (sizeof "!") - 1, 1 },
  { "nw", "new", (sizeof "new") - 1, 1 },
  { "oR", "|=", (sizeof "|=") - 1, 2 },
  { "oo", "||", (sizeof "||") - 1, 2 },
  { "or", "|", (sizeof "|") - 1, 2 },
  { "pL", "+=", (sizeof "+=") - 1, 2 },
  { "pl", "+", (sizeof "+") - 1, 2 },
  { "pm", "->*", (sizeof "->*") - 1, 2 },
  { "pp", "++", (sizeof "++") - 1, 1 },
  { "ps", "+", (sizeof "+") - 1, 1 },
  { "pt", "->", (sizeof "->") - 1, 2 },
  { "qu", "?", (sizeof "?") - 1, 3 },
  { "rM", "%=", (sizeof "%=") - 1, 2 },
  { "rS", ">>=", (sizeof ">>=") - 1, 2 },
  { "rm", "%", (sizeof "%") - 1, 2 },
  { "rs", ">>", (sizeof ">>") - 1, 2 },
  { "st", "sizeof ", (sizeof "sizeof ") - 1, 1 },
  { "sz", "sizeof ", (sizeof "sizeof ") - 1, 1 },
  { ((void *)0), ((void *)0), 0, 0 }
};
static struct demangle_component *
d_operator_name (di)
     struct d_info *di;
{
  char c1;
  char c2;
  c1 = (*((di)->n++));
  c2 = (*((di)->n++));
  if (c1 == 'v' && ((c2) >= '0' && (c2) <= '9'))
    return d_make_extended_operator (di, c2 - '0', d_source_name (di));
  else if (c1 == 'c' && c2 == 'v')
    return d_make_comp (di, DEMANGLE_COMPONENT_CAST,
   cplus_demangle_type (di), ((void *)0));
  else
    {
      int low = 0;
      int high = ((sizeof (cplus_demangle_operators)
     / sizeof (cplus_demangle_operators[0]))
    - 1);
      while (1)
 {
   int i;
   const struct demangle_operator_info *p;
   i = low + (high - low) / 2;
   p = cplus_demangle_operators + i;
   if (c1 == p->code[0] && c2 == p->code[1])
     return d_make_operator (di, p);
   if (c1 < p->code[0] || (c1 == p->code[0] && c2 < p->code[1]))
     high = i;
   else
     low = i + 1;
   if (low == high)
     return ((void *)0);
 }
    }
}
static struct demangle_component *
d_special_name (di)
     struct d_info *di;
{
  char c;
  di->expansion += 20;
  c = (*((di)->n++));
  if (c == 'T')
    {
      switch ((*((di)->n++)))
 {
 case 'V':
   di->expansion -= 5;
   return d_make_comp (di, DEMANGLE_COMPONENT_VTABLE,
         cplus_demangle_type (di), ((void *)0));
 case 'T':
   di->expansion -= 10;
   return d_make_comp (di, DEMANGLE_COMPONENT_VTT,
         cplus_demangle_type (di), ((void *)0));
 case 'I':
   return d_make_comp (di, DEMANGLE_COMPONENT_TYPEINFO,
         cplus_demangle_type (di), ((void *)0));
 case 'S':
   return d_make_comp (di, DEMANGLE_COMPONENT_TYPEINFO_NAME,
         cplus_demangle_type (di), ((void *)0));
 case 'h':
   if (! d_call_offset (di, 'h'))
     return ((void *)0);
   return d_make_comp (di, DEMANGLE_COMPONENT_THUNK,
         d_encoding (di, 0), ((void *)0));
 case 'v':
   if (! d_call_offset (di, 'v'))
     return ((void *)0);
   return d_make_comp (di, DEMANGLE_COMPONENT_VIRTUAL_THUNK,
         d_encoding (di, 0), ((void *)0));
 case 'c':
   if (! d_call_offset (di, 0))
     return ((void *)0);
   if (! d_call_offset (di, 0))
     return ((void *)0);
   return d_make_comp (di, DEMANGLE_COMPONENT_COVARIANT_THUNK,
         d_encoding (di, 0), ((void *)0));
 case 'C':
   {
     struct demangle_component *derived_type;
     long offset;
     struct demangle_component *base_type;
     derived_type = cplus_demangle_type (di);
     offset = d_number (di);
     if (offset < 0)
       return ((void *)0);
     if ((*((di)->n++)) != '_')
       return ((void *)0);
     base_type = cplus_demangle_type (di);
     di->expansion += 5;
     return d_make_comp (di, DEMANGLE_COMPONENT_CONSTRUCTION_VTABLE,
    base_type, derived_type);
   }
 case 'F':
   return d_make_comp (di, DEMANGLE_COMPONENT_TYPEINFO_FN,
         cplus_demangle_type (di), ((void *)0));
 case 'J':
   return d_make_comp (di, DEMANGLE_COMPONENT_JAVA_CLASS,
         cplus_demangle_type (di), ((void *)0));
 default:
   return ((void *)0);
 }
    }
  else if (c == 'G')
    {
      switch ((*((di)->n++)))
 {
 case 'V':
   return d_make_comp (di, DEMANGLE_COMPONENT_GUARD, d_name (di), ((void *)0));
 case 'R':
   return d_make_comp (di, DEMANGLE_COMPONENT_REFTEMP, d_name (di),
         ((void *)0));
 default:
   return ((void *)0);
 }
    }
  else
    return ((void *)0);
}
static int
d_call_offset (di, c)
     struct d_info *di;
     int c;
{
  long offset;
  long virtual_offset;
  if (c == 0)
    c = (*((di)->n++));
  if (c == 'h')
    offset = d_number (di);
  else if (c == 'v')
    {
      offset = d_number (di);
      if ((*((di)->n++)) != '_')
 return 0;
      virtual_offset = d_number (di);
    }
  else
    return 0;
  if ((*((di)->n++)) != '_')
    return 0;
  return 1;
}
static struct demangle_component *
d_ctor_dtor_name (di)
     struct d_info *di;
{
  if (di->last_name != ((void *)0))
    {
      if (di->last_name->type == DEMANGLE_COMPONENT_NAME)
 di->expansion += di->last_name->u.s_name.len;
      else if (di->last_name->type == DEMANGLE_COMPONENT_SUB_STD)
 di->expansion += di->last_name->u.s_string.len;
    }
  switch ((*((di)->n++)))
    {
    case 'C':
      {
 enum gnu_v3_ctor_kinds kind;
 switch ((*((di)->n++)))
   {
   case '1':
     kind = gnu_v3_complete_object_ctor;
     break;
   case '2':
     kind = gnu_v3_base_object_ctor;
     break;
   case '3':
     kind = gnu_v3_complete_object_allocating_ctor;
     break;
   default:
     return ((void *)0);
   }
 return d_make_ctor (di, kind, di->last_name);
      }
    case 'D':
      {
 enum gnu_v3_dtor_kinds kind;
 switch ((*((di)->n++)))
   {
   case '0':
     kind = gnu_v3_deleting_dtor;
     break;
   case '1':
     kind = gnu_v3_complete_object_dtor;
     break;
   case '2':
     kind = gnu_v3_base_object_dtor;
     break;
   default:
     return ((void *)0);
   }
 return d_make_dtor (di, kind, di->last_name);
      }
    default:
      return ((void *)0);
    }
}
const struct demangle_builtin_type_info
cplus_demangle_builtin_types[(26)] =
{
          { "signed char", (sizeof "signed char") - 1, "signed char", (sizeof "signed char") - 1, D_PRINT_DEFAULT },
          { "bool", (sizeof "bool") - 1, "boolean", (sizeof "boolean") - 1, D_PRINT_BOOL },
          { "char", (sizeof "char") - 1, "byte", (sizeof "byte") - 1, D_PRINT_DEFAULT },
          { "double", (sizeof "double") - 1, "double", (sizeof "double") - 1, D_PRINT_FLOAT },
          { "long double", (sizeof "long double") - 1, "long double", (sizeof "long double") - 1, D_PRINT_FLOAT },
          { "float", (sizeof "float") - 1, "float", (sizeof "float") - 1, D_PRINT_FLOAT },
          { "__float128", (sizeof "__float128") - 1, "__float128", (sizeof "__float128") - 1, D_PRINT_FLOAT },
          { "unsigned char", (sizeof "unsigned char") - 1, "unsigned char", (sizeof "unsigned char") - 1, D_PRINT_DEFAULT },
          { "int", (sizeof "int") - 1, "int", (sizeof "int") - 1, D_PRINT_INT },
          { "unsigned int", (sizeof "unsigned int") - 1, "unsigned", (sizeof "unsigned") - 1, D_PRINT_UNSIGNED },
          { ((void *)0), 0, ((void *)0), 0, D_PRINT_DEFAULT },
          { "long", (sizeof "long") - 1, "long", (sizeof "long") - 1, D_PRINT_LONG },
          { "unsigned long", (sizeof "unsigned long") - 1, "unsigned long", (sizeof "unsigned long") - 1, D_PRINT_UNSIGNED_LONG },
          { "__int128", (sizeof "__int128") - 1, "__int128", (sizeof "__int128") - 1, D_PRINT_DEFAULT },
          { "unsigned __int128", (sizeof "unsigned __int128") - 1, "unsigned __int128", (sizeof "unsigned __int128") - 1,
     D_PRINT_DEFAULT },
          { ((void *)0), 0, ((void *)0), 0, D_PRINT_DEFAULT },
          { ((void *)0), 0, ((void *)0), 0, D_PRINT_DEFAULT },
          { ((void *)0), 0, ((void *)0), 0, D_PRINT_DEFAULT },
          { "short", (sizeof "short") - 1, "short", (sizeof "short") - 1, D_PRINT_DEFAULT },
          { "unsigned short", (sizeof "unsigned short") - 1, "unsigned short", (sizeof "unsigned short") - 1, D_PRINT_DEFAULT },
          { ((void *)0), 0, ((void *)0), 0, D_PRINT_DEFAULT },
          { "void", (sizeof "void") - 1, "void", (sizeof "void") - 1, D_PRINT_VOID },
          { "wchar_t", (sizeof "wchar_t") - 1, "char", (sizeof "char") - 1, D_PRINT_DEFAULT },
          { "long long", (sizeof "long long") - 1, "long", (sizeof "long") - 1, D_PRINT_LONG_LONG },
          { "unsigned long long", (sizeof "unsigned long long") - 1, "unsigned long long", (sizeof "unsigned long long") - 1,
     D_PRINT_UNSIGNED_LONG_LONG },
          { "...", (sizeof "...") - 1, "...", (sizeof "...") - 1, D_PRINT_DEFAULT },
};
struct demangle_component *
cplus_demangle_type (di)
     struct d_info *di;
{
  char peek;
  struct demangle_component *ret;
  int can_subst;
  peek = (*((di)->n));
  if (peek == 'r' || peek == 'V' || peek == 'K')
    {
      struct demangle_component *pret;
      pret = d_cv_qualifiers (di, &ret, 0);
      if (pret == ((void *)0))
 return ((void *)0);
      *pret = cplus_demangle_type (di);
      if (! d_add_substitution (di, ret))
 return ((void *)0);
      return ret;
    }
  can_subst = 1;
  switch (peek)
    {
    case 'a': case 'b': case 'c': case 'd': case 'e': case 'f': case 'g':
    case 'h': case 'i': case 'j': case 'l': case 'm': case 'n':
    case 'o': case 's': case 't':
    case 'v': case 'w': case 'x': case 'y': case 'z':
      ret = d_make_builtin_type (di,
     &cplus_demangle_builtin_types[peek - 'a']);
      di->expansion += ret->u.s_builtin.type->len;
      can_subst = 0;
      ((di)->n += (1));
      break;
    case 'u':
      ((di)->n += (1));
      ret = d_make_comp (di, DEMANGLE_COMPONENT_VENDOR_TYPE,
    d_source_name (di), ((void *)0));
      break;
    case 'F':
      ret = d_function_type (di);
      break;
    case '0': case '1': case '2': case '3': case '4':
    case '5': case '6': case '7': case '8': case '9':
    case 'N':
    case 'Z':
      ret = d_class_enum_type (di);
      break;
    case 'A':
      ret = d_array_type (di);
      break;
    case 'M':
      ret = d_pointer_to_member_type (di);
      break;
    case 'T':
      ret = d_template_param (di);
      if ((*((di)->n)) == 'I')
 {
   if (! d_add_substitution (di, ret))
     return ((void *)0);
   ret = d_make_comp (di, DEMANGLE_COMPONENT_TEMPLATE, ret,
        d_template_args (di));
 }
      break;
    case 'S':
      {
 char peek_next;
 peek_next = ((di)->n[1]);
 if (((peek_next) >= '0' && (peek_next) <= '9')
     || peek_next == '_'
     || ((peek_next) >= 'A' && (peek_next) <= 'Z'))
   {
     ret = d_substitution (di, 0);
     if ((*((di)->n)) == 'I')
       ret = d_make_comp (di, DEMANGLE_COMPONENT_TEMPLATE, ret,
     d_template_args (di));
     else
       can_subst = 0;
   }
 else
   {
     ret = d_class_enum_type (di);
     if (ret != ((void *)0) && ret->type == DEMANGLE_COMPONENT_SUB_STD)
       can_subst = 0;
   }
      }
      break;
    case 'P':
      ((di)->n += (1));
      ret = d_make_comp (di, DEMANGLE_COMPONENT_POINTER,
    cplus_demangle_type (di), ((void *)0));
      break;
    case 'R':
      ((di)->n += (1));
      ret = d_make_comp (di, DEMANGLE_COMPONENT_REFERENCE,
    cplus_demangle_type (di), ((void *)0));
      break;
    case 'C':
      ((di)->n += (1));
      ret = d_make_comp (di, DEMANGLE_COMPONENT_COMPLEX,
    cplus_demangle_type (di), ((void *)0));
      break;
    case 'G':
      ((di)->n += (1));
      ret = d_make_comp (di, DEMANGLE_COMPONENT_IMAGINARY,
    cplus_demangle_type (di), ((void *)0));
      break;
    case 'U':
      ((di)->n += (1));
      ret = d_source_name (di);
      ret = d_make_comp (di, DEMANGLE_COMPONENT_VENDOR_TYPE_QUAL,
    cplus_demangle_type (di), ret);
      break;
    default:
      return ((void *)0);
    }
  if (can_subst)
    {
      if (! d_add_substitution (di, ret))
 return ((void *)0);
    }
  return ret;
}
static struct demangle_component *
d_cv_qualifiers (di, pret, member_fn)
     struct d_info *di;
     struct demangle_component *pret;
     int member_fn;
{
  char peek;
  peek = (*((di)->n));
  while (peek == 'r' || peek == 'V' || peek == 'K')
    {
      enum demangle_component_type t;
      ((di)->n += (1));
      if (peek == 'r')
 {
   t = (member_fn
        ? DEMANGLE_COMPONENT_RESTRICT_THIS
        : DEMANGLE_COMPONENT_RESTRICT);
   di->expansion += sizeof "restrict";
 }
      else if (peek == 'V')
 {
   t = (member_fn
        ? DEMANGLE_COMPONENT_VOLATILE_THIS
        : DEMANGLE_COMPONENT_VOLATILE);
   di->expansion += sizeof "volatile";
 }
      else
 {
   t = (member_fn
        ? DEMANGLE_COMPONENT_CONST_THIS
        : DEMANGLE_COMPONENT_CONST);
   di->expansion += sizeof "const";
 }
      *pret = d_make_comp (di, t, ((void *)0), ((void *)0));
      if (*pret == ((void *)0))
 return ((void *)0);
      pret = &((*pret)->u.s_binary.left);
      peek = (*((di)->n));
    }
  return pret;
}
static struct demangle_component *
d_function_type (di)
     struct d_info *di;
{
  struct demangle_component *ret;
  if ((*((di)->n++)) != 'F')
    return ((void *)0);
  if ((*((di)->n)) == 'Y')
    {
      ((di)->n += (1));
    }
  ret = d_bare_function_type (di, 1);
  if ((*((di)->n++)) != 'E')
    return ((void *)0);
  return ret;
}
static struct demangle_component *
d_bare_function_type (di, has_return_type)
     struct d_info *di;
     int has_return_type;
{
  struct demangle_component *return_type;
  struct demangle_component *tl;
  struct demangle_component *ptl;
  return_type = ((void *)0);
  tl = ((void *)0);
  ptl = &tl;
  while (1)
    {
      char peek;
      struct demangle_component *type;
      peek = (*((di)->n));
      if (peek == 0 || peek == 'E')
 break;
      type = cplus_demangle_type (di);
      if (type == ((void *)0))
 return ((void *)0);
      if (has_return_type)
 {
   return_type = type;
   has_return_type = 0;
 }
      else
 {
   *ptl = d_make_comp (di, DEMANGLE_COMPONENT_ARGLIST, type, ((void *)0));
   if (*ptl == ((void *)0))
     return ((void *)0);
   ptl = &((*ptl)->u.s_binary.right);
 }
    }
  if (tl == ((void *)0))
    return ((void *)0);
  if (((tl)->u.s_binary.right) == ((void *)0)
      && ((tl)->u.s_binary.left)->type == DEMANGLE_COMPONENT_BUILTIN_TYPE
      && ((tl)->u.s_binary.left)->u.s_builtin.type->print == D_PRINT_VOID)
    {
      di->expansion -= ((tl)->u.s_binary.left)->u.s_builtin.type->len;
      tl = ((void *)0);
    }
  return d_make_comp (di, DEMANGLE_COMPONENT_FUNCTION_TYPE, return_type, tl);
}
static struct demangle_component *
d_class_enum_type (di)
     struct d_info *di;
{
  return d_name (di);
}
static struct demangle_component *
d_array_type (di)
     struct d_info *di;
{
  char peek;
  struct demangle_component *dim;
  if ((*((di)->n++)) != 'A')
    return ((void *)0);
  peek = (*((di)->n));
  if (peek == '_')
    dim = ((void *)0);
  else if (((peek) >= '0' && (peek) <= '9'))
    {
      const char *s;
      s = ((di)->n);
      do
 {
   ((di)->n += (1));
   peek = (*((di)->n));
 }
      while (((peek) >= '0' && (peek) <= '9'));
      dim = d_make_name (di, s, ((di)->n) - s);
      if (dim == ((void *)0))
 return ((void *)0);
    }
  else
    {
      dim = d_expression (di);
      if (dim == ((void *)0))
 return ((void *)0);
    }
  if ((*((di)->n++)) != '_')
    return ((void *)0);
  return d_make_comp (di, DEMANGLE_COMPONENT_ARRAY_TYPE, dim,
        cplus_demangle_type (di));
}
static struct demangle_component *
d_pointer_to_member_type (di)
     struct d_info *di;
{
  struct demangle_component *cl;
  struct demangle_component *mem;
  struct demangle_component *pmem;
  if ((*((di)->n++)) != 'M')
    return ((void *)0);
  cl = cplus_demangle_type (di);
  pmem = d_cv_qualifiers (di, &mem, 1);
  if (pmem == ((void *)0))
    return ((void *)0);
  *pmem = cplus_demangle_type (di);
  return d_make_comp (di, DEMANGLE_COMPONENT_PTRMEM_TYPE, cl, mem);
}
static struct demangle_component *
d_template_param (di)
     struct d_info *di;
{
  long param;
  if ((*((di)->n++)) != 'T')
    return ((void *)0);
  if ((*((di)->n)) == '_')
    param = 0;
  else
    {
      param = d_number (di);
      if (param < 0)
 return ((void *)0);
      param += 1;
    }
  if ((*((di)->n++)) != '_')
    return ((void *)0);
  ++di->did_subs;
  return d_make_template_param (di, param);
}
static struct demangle_component *
d_template_args (di)
     struct d_info *di;
{
  struct demangle_component *hold_last_name;
  struct demangle_component *al;
  struct demangle_component *pal;
  hold_last_name = di->last_name;
  if ((*((di)->n++)) != 'I')
    return ((void *)0);
  al = ((void *)0);
  pal = &al;
  while (1)
    {
      struct demangle_component *a;
      a = d_template_arg (di);
      if (a == ((void *)0))
 return ((void *)0);
      *pal = d_make_comp (di, DEMANGLE_COMPONENT_TEMPLATE_ARGLIST, a, ((void *)0));
      if (*pal == ((void *)0))
 return ((void *)0);
      pal = &((*pal)->u.s_binary.right);
      if ((*((di)->n)) == 'E')
 {
   ((di)->n += (1));
   break;
 }
    }
  di->last_name = hold_last_name;
  return al;
}
static struct demangle_component *
d_template_arg (di)
     struct d_info *di;
{
  struct demangle_component *ret;
  switch ((*((di)->n)))
    {
    case 'X':
      ((di)->n += (1));
      ret = d_expression (di);
      if ((*((di)->n++)) != 'E')
 return ((void *)0);
      return ret;
    case 'L':
      return d_expr_primary (di);
    default:
      return cplus_demangle_type (di);
    }
}
static struct demangle_component *
d_expression (di)
     struct d_info *di;
{
  char peek;
  peek = (*((di)->n));
  if (peek == 'L')
    return d_expr_primary (di);
  else if (peek == 'T')
    return d_template_param (di);
  else if (peek == 's' && ((di)->n[1]) == 'r')
    {
      struct demangle_component *type;
      struct demangle_component *name;
      ((di)->n += (2));
      type = cplus_demangle_type (di);
      name = d_unqualified_name (di);
      if ((*((di)->n)) != 'I')
 return d_make_comp (di, DEMANGLE_COMPONENT_QUAL_NAME, type, name);
      else
 return d_make_comp (di, DEMANGLE_COMPONENT_QUAL_NAME, type,
       d_make_comp (di, DEMANGLE_COMPONENT_TEMPLATE, name,
      d_template_args (di)));
    }
  else
    {
      struct demangle_component *op;
      int args;
      op = d_operator_name (di);
      if (op == ((void *)0))
 return ((void *)0);
      if (op->type == DEMANGLE_COMPONENT_OPERATOR)
 di->expansion += op->u.s_operator.op->len - 2;
      if (op->type == DEMANGLE_COMPONENT_OPERATOR
   && strcmp (op->u.s_operator.op->code, "st") == 0)
 return d_make_comp (di, DEMANGLE_COMPONENT_UNARY, op,
       cplus_demangle_type (di));
      switch (op->type)
 {
 default:
   return ((void *)0);
 case DEMANGLE_COMPONENT_OPERATOR:
   args = op->u.s_operator.op->args;
   break;
 case DEMANGLE_COMPONENT_EXTENDED_OPERATOR:
   args = op->u.s_extended_operator.args;
   break;
 case DEMANGLE_COMPONENT_CAST:
   args = 1;
   break;
 }
      switch (args)
 {
 case 1:
   return d_make_comp (di, DEMANGLE_COMPONENT_UNARY, op,
         d_expression (di));
 case 2:
   {
     struct demangle_component *left;
     left = d_expression (di);
     return d_make_comp (di, DEMANGLE_COMPONENT_BINARY, op,
    d_make_comp (di,
          DEMANGLE_COMPONENT_BINARY_ARGS,
          left,
          d_expression (di)));
   }
 case 3:
   {
     struct demangle_component *first;
     struct demangle_component *second;
     first = d_expression (di);
     second = d_expression (di);
     return d_make_comp (di, DEMANGLE_COMPONENT_TRINARY, op,
    d_make_comp (di,
          DEMANGLE_COMPONENT_TRINARY_ARG1,
          first,
          d_make_comp (di,
         DEMANGLE_COMPONENT_TRINARY_ARG2,
         second,
         d_expression (di))));
   }
 default:
   return ((void *)0);
 }
    }
}
static struct demangle_component *
d_expr_primary (di)
     struct d_info *di;
{
  struct demangle_component *ret;
  if ((*((di)->n++)) != 'L')
    return ((void *)0);
  if ((*((di)->n)) == '_')
    ret = cplus_demangle_mangled_name (di, 0);
  else
    {
      struct demangle_component *type;
      enum demangle_component_type t;
      const char *s;
      type = cplus_demangle_type (di);
      if (type == ((void *)0))
 return ((void *)0);
      if (type->type == DEMANGLE_COMPONENT_BUILTIN_TYPE
   && type->u.s_builtin.type->print != D_PRINT_DEFAULT)
 di->expansion -= type->u.s_builtin.type->len;
      t = DEMANGLE_COMPONENT_LITERAL;
      if ((*((di)->n)) == 'n')
 {
   t = DEMANGLE_COMPONENT_LITERAL_NEG;
   ((di)->n += (1));
 }
      s = ((di)->n);
      while ((*((di)->n)) != 'E')
 ((di)->n += (1));
      ret = d_make_comp (di, t, type, d_make_name (di, s, ((di)->n) - s));
    }
  if ((*((di)->n++)) != 'E')
    return ((void *)0);
  return ret;
}
static struct demangle_component *
d_local_name (di)
     struct d_info *di;
{
  struct demangle_component *function;
  if ((*((di)->n++)) != 'Z')
    return ((void *)0);
  function = d_encoding (di, 0);
  if ((*((di)->n++)) != 'E')
    return ((void *)0);
  if ((*((di)->n)) == 's')
    {
      ((di)->n += (1));
      if (! d_discriminator (di))
 return ((void *)0);
      return d_make_comp (di, DEMANGLE_COMPONENT_LOCAL_NAME, function,
     d_make_name (di, "string literal",
           sizeof "string literal" - 1));
    }
  else
    {
      struct demangle_component *name;
      name = d_name (di);
      if (! d_discriminator (di))
 return ((void *)0);
      return d_make_comp (di, DEMANGLE_COMPONENT_LOCAL_NAME, function, name);
    }
}
static int
d_discriminator (di)
     struct d_info *di;
{
  long discrim;
  if ((*((di)->n)) != '_')
    return 1;
  ((di)->n += (1));
  discrim = d_number (di);
  if (discrim < 0)
    return 0;
  return 1;
}
static int
d_add_substitution (di, dc)
     struct d_info *di;
     struct demangle_component *dc;
{
  if (dc == ((void *)0))
    return 0;
  if (di->next_sub >= di->num_subs)
    return 0;
  di->subs[di->next_sub] = dc;
  ++di->next_sub;
  return 1;
}
static const struct d_standard_sub_info standard_subs[] =
{
  { 't', "std", (sizeof "std") - 1,
    "std", (sizeof "std") - 1,
    ((void *)0), 0 },
  { 'a', "std::allocator", (sizeof "std::allocator") - 1,
    "std::allocator", (sizeof "std::allocator") - 1,
    "allocator", (sizeof "allocator") - 1 },
  { 'b', "std::basic_string", (sizeof "std::basic_string") - 1,
    "std::basic_string", (sizeof "std::basic_string") - 1,
    "basic_string", (sizeof "basic_string") - 1 },
  { 's', "std::string", (sizeof "std::string") - 1,
    "std::basic_string<char, std::char_traits<char>, std::allocator<char> >", (sizeof "std::basic_string<char, std::char_traits<char>, std::allocator<char> >") - 1,
    "basic_string", (sizeof "basic_string") - 1 },
  { 'i', "std::istream", (sizeof "std::istream") - 1,
    "std::basic_istream<char, std::char_traits<char> >", (sizeof "std::basic_istream<char, std::char_traits<char> >") - 1,
    "basic_istream", (sizeof "basic_istream") - 1 },
  { 'o', "std::ostream", (sizeof "std::ostream") - 1,
    "std::basic_ostream<char, std::char_traits<char> >", (sizeof "std::basic_ostream<char, std::char_traits<char> >") - 1,
    "basic_ostream", (sizeof "basic_ostream") - 1 },
  { 'd', "std::iostream", (sizeof "std::iostream") - 1,
    "std::basic_iostream<char, std::char_traits<char> >", (sizeof "std::basic_iostream<char, std::char_traits<char> >") - 1,
    "basic_iostream", (sizeof "basic_iostream") - 1 }
};
static struct demangle_component *
d_substitution (di, prefix)
     struct d_info *di;
     int prefix;
{
  char c;
  if ((*((di)->n++)) != 'S')
    return ((void *)0);
  c = (*((di)->n++));
  if (c == '_' || ((c) >= '0' && (c) <= '9') || ((c) >= 'A' && (c) <= 'Z'))
    {
      int id;
      id = 0;
      if (c != '_')
 {
   do
     {
       if (((c) >= '0' && (c) <= '9'))
  id = id * 36 + c - '0';
       else if (((c) >= 'A' && (c) <= 'Z'))
  id = id * 36 + c - 'A' + 10;
       else
  return ((void *)0);
       c = (*((di)->n++));
     }
   while (c != '_');
   ++id;
 }
      if (id >= di->next_sub)
 return ((void *)0);
      ++di->did_subs;
      return di->subs[id];
    }
  else
    {
      int verbose;
      const struct d_standard_sub_info *p;
      const struct d_standard_sub_info *pend;
      verbose = (di->options & (1 << 3)) != 0;
      if (! verbose && prefix)
 {
   char peek;
   peek = (*((di)->n));
   if (peek == 'C' || peek == 'D')
     verbose = 1;
 }
      pend = (&standard_subs[0]
       + sizeof standard_subs / sizeof standard_subs[0]);
      for (p = &standard_subs[0]; p < pend; ++p)
 {
   if (c == p->code)
     {
       const char *s;
       int len;
       if (p->set_last_name != ((void *)0))
  di->last_name = d_make_sub (di, p->set_last_name,
         p->set_last_name_len);
       if (verbose)
  {
    s = p->full_expansion;
    len = p->full_len;
  }
       else
  {
    s = p->simple_expansion;
    len = p->simple_len;
  }
       di->expansion += len;
       return d_make_sub (di, s, len);
     }
 }
      return ((void *)0);
    }
}
static void
d_print_resize (dpi, add)
     struct d_print_info *dpi;
     size_t add;
{
  size_t need;
  if (dpi->buf == ((void *)0))
    return;
  need = dpi->len + add;
  while (need > dpi->alc)
    {
      size_t newalc;
      char *newbuf;
      newalc = dpi->alc * 2;
      newbuf = realloc (dpi->buf, newalc);
      if (newbuf == ((void *)0))
 {
   free (dpi->buf);
   dpi->buf = ((void *)0);
   dpi->allocation_failure = 1;
   return;
 }
      dpi->buf = newbuf;
      dpi->alc = newalc;
    }
}
static void
d_print_append_char (dpi, c)
     struct d_print_info *dpi;
     int c;
{
  if (dpi->buf != ((void *)0))
    {
      if (dpi->len >= dpi->alc)
 {
   d_print_resize (dpi, 1);
   if (dpi->buf == ((void *)0))
     return;
 }
      dpi->buf[dpi->len] = c;
      ++dpi->len;
    }
}
static void
d_print_append_buffer (dpi, s, l)
     struct d_print_info *dpi;
     const char *s;
     size_t l;
{
  if (dpi->buf != ((void *)0))
    {
      if (dpi->len + l > dpi->alc)
 {
   d_print_resize (dpi, l);
   if (dpi->buf == ((void *)0))
     return;
 }
      memcpy (dpi->buf + dpi->len, s, l);
      dpi->len += l;
    }
}
static void
d_print_error (dpi)
     struct d_print_info *dpi;
{
  free (dpi->buf);
  dpi->buf = ((void *)0);
}
char *
cplus_demangle_print (options, dc, estimate, palc)
     int options;
     const struct demangle_component *dc;
     int estimate;
     size_t *palc;
{
  struct d_print_info dpi;
  dpi.options = options;
  dpi.alc = estimate + 1;
  dpi.buf = malloc (dpi.alc);
  if (dpi.buf == ((void *)0))
    {
      *palc = 1;
      return ((void *)0);
    }
  dpi.len = 0;
  dpi.templates = ((void *)0);
  dpi.modifiers = ((void *)0);
  dpi.allocation_failure = 0;
  d_print_comp (&dpi, dc);
  do { if ((&dpi)->buf != ((void *)0) && (&dpi)->len < (&dpi)->alc) (&dpi)->buf[(&dpi)->len++] = (0); else d_print_append_char ((&dpi), (0)); } while (0);
  if (dpi.buf != ((void *)0))
    *palc = dpi.alc;
  else
    *palc = dpi.allocation_failure;
  return dpi.buf;
}
static void
d_print_comp (dpi, dc)
     struct d_print_info *dpi;
     const struct demangle_component *dc;
{
  if (dc == ((void *)0))
    {
      d_print_error (dpi);
      return;
    }
  if (((dpi)->buf == ((void *)0)))
    return;
  switch (dc->type)
    {
    case DEMANGLE_COMPONENT_NAME:
      if ((dpi->options & (1 << 2)) == 0)
 do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (dc->u.s_name.len) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, (dc->u.s_name.s), (dc->u.s_name.len)); (dpi)->len += dc->u.s_name.len; } else d_print_append_buffer ((dpi), (dc->u.s_name.s), (dc->u.s_name.len)); } while (0);
      else
 d_print_java_identifier (dpi, dc->u.s_name.s, dc->u.s_name.len);
      return;
    case DEMANGLE_COMPONENT_QUAL_NAME:
    case DEMANGLE_COMPONENT_LOCAL_NAME:
      d_print_comp (dpi, ((dc)->u.s_binary.left));
      if ((dpi->options & (1 << 2)) == 0)
 do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (sizeof ("::") - 1) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, (("::")), (sizeof ("::") - 1)); (dpi)->len += sizeof ("::") - 1; } else d_print_append_buffer ((dpi), (("::")), (sizeof ("::") - 1)); } while (0);
      else
 do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = ('.'); else d_print_append_char ((dpi), ('.')); } while (0);
      d_print_comp (dpi, ((dc)->u.s_binary.right));
      return;
    case DEMANGLE_COMPONENT_TYPED_NAME:
      {
 struct d_print_mod *hold_modifiers;
 struct demangle_component *typed_name;
 struct d_print_mod adpm[4];
 unsigned int i;
 struct d_print_template dpt;
 hold_modifiers = dpi->modifiers;
 i = 0;
 typed_name = ((dc)->u.s_binary.left);
 while (typed_name != ((void *)0))
   {
     if (i >= sizeof adpm / sizeof adpm[0])
       {
  d_print_error (dpi);
  return;
       }
     adpm[i].next = dpi->modifiers;
     dpi->modifiers = &adpm[i];
     adpm[i].mod = typed_name;
     adpm[i].printed = 0;
     adpm[i].templates = dpi->templates;
     ++i;
     if (typed_name->type != DEMANGLE_COMPONENT_RESTRICT_THIS
  && typed_name->type != DEMANGLE_COMPONENT_VOLATILE_THIS
  && typed_name->type != DEMANGLE_COMPONENT_CONST_THIS)
       break;
     typed_name = ((typed_name)->u.s_binary.left);
   }
 if (typed_name->type == DEMANGLE_COMPONENT_TEMPLATE)
   {
     dpt.next = dpi->templates;
     dpi->templates = &dpt;
     dpt.template = typed_name;
   }
 if (typed_name->type == DEMANGLE_COMPONENT_LOCAL_NAME)
   {
     struct demangle_component *local_name;
     local_name = ((typed_name)->u.s_binary.right);
     while (local_name->type == DEMANGLE_COMPONENT_RESTRICT_THIS
     || local_name->type == DEMANGLE_COMPONENT_VOLATILE_THIS
     || local_name->type == DEMANGLE_COMPONENT_CONST_THIS)
       {
  if (i >= sizeof adpm / sizeof adpm[0])
    {
      d_print_error (dpi);
      return;
    }
  adpm[i] = adpm[i - 1];
  adpm[i].next = &adpm[i - 1];
  dpi->modifiers = &adpm[i];
  adpm[i - 1].mod = local_name;
  adpm[i - 1].printed = 0;
  adpm[i - 1].templates = dpi->templates;
  ++i;
  local_name = ((local_name)->u.s_binary.left);
       }
   }
 d_print_comp (dpi, ((dc)->u.s_binary.right));
 if (typed_name->type == DEMANGLE_COMPONENT_TEMPLATE)
   dpi->templates = dpt.next;
 while (i > 0)
   {
     --i;
     if (! adpm[i].printed)
       {
  do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = (' '); else d_print_append_char ((dpi), (' ')); } while (0);
  d_print_mod (dpi, adpm[i].mod);
       }
   }
 dpi->modifiers = hold_modifiers;
 return;
      }
    case DEMANGLE_COMPONENT_TEMPLATE:
      {
 struct d_print_mod *hold_dpm;
 hold_dpm = dpi->modifiers;
 dpi->modifiers = ((void *)0);
 d_print_comp (dpi, ((dc)->u.s_binary.left));
 if (((dpi)->buf == ((void *)0) || (dpi)->len == 0 ? 0 : (dpi)->buf[(dpi)->len - 1]) == '<')
   do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = (' '); else d_print_append_char ((dpi), (' ')); } while (0);
 do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = ('<'); else d_print_append_char ((dpi), ('<')); } while (0);
 d_print_comp (dpi, ((dc)->u.s_binary.right));
 if (((dpi)->buf == ((void *)0) || (dpi)->len == 0 ? 0 : (dpi)->buf[(dpi)->len - 1]) == '>')
   do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = (' '); else d_print_append_char ((dpi), (' ')); } while (0);
 do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = ('>'); else d_print_append_char ((dpi), ('>')); } while (0);
 dpi->modifiers = hold_dpm;
 return;
      }
    case DEMANGLE_COMPONENT_TEMPLATE_PARAM:
      {
 long i;
 struct demangle_component *a;
 struct d_print_template *hold_dpt;
 if (dpi->templates == ((void *)0))
   {
     d_print_error (dpi);
     return;
   }
 i = dc->u.s_number.number;
 for (a = ((dpi->templates->template)->u.s_binary.right);
      a != ((void *)0);
      a = ((a)->u.s_binary.right))
   {
     if (a->type != DEMANGLE_COMPONENT_TEMPLATE_ARGLIST)
       {
  d_print_error (dpi);
  return;
       }
     if (i <= 0)
       break;
     --i;
   }
 if (i != 0 || a == ((void *)0))
   {
     d_print_error (dpi);
     return;
   }
 hold_dpt = dpi->templates;
 dpi->templates = hold_dpt->next;
 d_print_comp (dpi, ((a)->u.s_binary.left));
 dpi->templates = hold_dpt;
 return;
      }
    case DEMANGLE_COMPONENT_CTOR:
      d_print_comp (dpi, dc->u.s_ctor.name);
      return;
    case DEMANGLE_COMPONENT_DTOR:
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = ('~'); else d_print_append_char ((dpi), ('~')); } while (0);
      d_print_comp (dpi, dc->u.s_dtor.name);
      return;
    case DEMANGLE_COMPONENT_VTABLE:
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (sizeof ("vtable for ") - 1) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, (("vtable for ")), (sizeof ("vtable for ") - 1)); (dpi)->len += sizeof ("vtable for ") - 1; } else d_print_append_buffer ((dpi), (("vtable for ")), (sizeof ("vtable for ") - 1)); } while (0);
      d_print_comp (dpi, ((dc)->u.s_binary.left));
      return;
    case DEMANGLE_COMPONENT_VTT:
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (sizeof ("VTT for ") - 1) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, (("VTT for ")), (sizeof ("VTT for ") - 1)); (dpi)->len += sizeof ("VTT for ") - 1; } else d_print_append_buffer ((dpi), (("VTT for ")), (sizeof ("VTT for ") - 1)); } while (0);
      d_print_comp (dpi, ((dc)->u.s_binary.left));
      return;
    case DEMANGLE_COMPONENT_CONSTRUCTION_VTABLE:
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (sizeof ("construction vtable for ") - 1) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, (("construction vtable for ")), (sizeof ("construction vtable for ") - 1)); (dpi)->len += sizeof ("construction vtable for ") - 1; } else d_print_append_buffer ((dpi), (("construction vtable for ")), (sizeof ("construction vtable for ") - 1)); } while (0);
      d_print_comp (dpi, ((dc)->u.s_binary.left));
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (sizeof ("-in-") - 1) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, (("-in-")), (sizeof ("-in-") - 1)); (dpi)->len += sizeof ("-in-") - 1; } else d_print_append_buffer ((dpi), (("-in-")), (sizeof ("-in-") - 1)); } while (0);
      d_print_comp (dpi, ((dc)->u.s_binary.right));
      return;
    case DEMANGLE_COMPONENT_TYPEINFO:
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (sizeof ("typeinfo for ") - 1) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, (("typeinfo for ")), (sizeof ("typeinfo for ") - 1)); (dpi)->len += sizeof ("typeinfo for ") - 1; } else d_print_append_buffer ((dpi), (("typeinfo for ")), (sizeof ("typeinfo for ") - 1)); } while (0);
      d_print_comp (dpi, ((dc)->u.s_binary.left));
      return;
    case DEMANGLE_COMPONENT_TYPEINFO_NAME:
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (sizeof ("typeinfo name for ") - 1) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, (("typeinfo name for ")), (sizeof ("typeinfo name for ") - 1)); (dpi)->len += sizeof ("typeinfo name for ") - 1; } else d_print_append_buffer ((dpi), (("typeinfo name for ")), (sizeof ("typeinfo name for ") - 1)); } while (0);
      d_print_comp (dpi, ((dc)->u.s_binary.left));
      return;
    case DEMANGLE_COMPONENT_TYPEINFO_FN:
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (sizeof ("typeinfo fn for ") - 1) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, (("typeinfo fn for ")), (sizeof ("typeinfo fn for ") - 1)); (dpi)->len += sizeof ("typeinfo fn for ") - 1; } else d_print_append_buffer ((dpi), (("typeinfo fn for ")), (sizeof ("typeinfo fn for ") - 1)); } while (0);
      d_print_comp (dpi, ((dc)->u.s_binary.left));
      return;
    case DEMANGLE_COMPONENT_THUNK:
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (sizeof ("non-virtual thunk to ") - 1) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, (("non-virtual thunk to ")), (sizeof ("non-virtual thunk to ") - 1)); (dpi)->len += sizeof ("non-virtual thunk to ") - 1; } else d_print_append_buffer ((dpi), (("non-virtual thunk to ")), (sizeof ("non-virtual thunk to ") - 1)); } while (0);
      d_print_comp (dpi, ((dc)->u.s_binary.left));
      return;
    case DEMANGLE_COMPONENT_VIRTUAL_THUNK:
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (sizeof ("virtual thunk to ") - 1) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, (("virtual thunk to ")), (sizeof ("virtual thunk to ") - 1)); (dpi)->len += sizeof ("virtual thunk to ") - 1; } else d_print_append_buffer ((dpi), (("virtual thunk to ")), (sizeof ("virtual thunk to ") - 1)); } while (0);
      d_print_comp (dpi, ((dc)->u.s_binary.left));
      return;
    case DEMANGLE_COMPONENT_COVARIANT_THUNK:
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (sizeof ("covariant return thunk to ") - 1) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, (("covariant return thunk to ")), (sizeof ("covariant return thunk to ") - 1)); (dpi)->len += sizeof ("covariant return thunk to ") - 1; } else d_print_append_buffer ((dpi), (("covariant return thunk to ")), (sizeof ("covariant return thunk to ") - 1)); } while (0);
      d_print_comp (dpi, ((dc)->u.s_binary.left));
      return;
    case DEMANGLE_COMPONENT_JAVA_CLASS:
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (sizeof ("java Class for ") - 1) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, (("java Class for ")), (sizeof ("java Class for ") - 1)); (dpi)->len += sizeof ("java Class for ") - 1; } else d_print_append_buffer ((dpi), (("java Class for ")), (sizeof ("java Class for ") - 1)); } while (0);
      d_print_comp (dpi, ((dc)->u.s_binary.left));
      return;
    case DEMANGLE_COMPONENT_GUARD:
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (sizeof ("guard variable for ") - 1) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, (("guard variable for ")), (sizeof ("guard variable for ") - 1)); (dpi)->len += sizeof ("guard variable for ") - 1; } else d_print_append_buffer ((dpi), (("guard variable for ")), (sizeof ("guard variable for ") - 1)); } while (0);
      d_print_comp (dpi, ((dc)->u.s_binary.left));
      return;
    case DEMANGLE_COMPONENT_REFTEMP:
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (sizeof ("reference temporary for ") - 1) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, (("reference temporary for ")), (sizeof ("reference temporary for ") - 1)); (dpi)->len += sizeof ("reference temporary for ") - 1; } else d_print_append_buffer ((dpi), (("reference temporary for ")), (sizeof ("reference temporary for ") - 1)); } while (0);
      d_print_comp (dpi, ((dc)->u.s_binary.left));
      return;
    case DEMANGLE_COMPONENT_SUB_STD:
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (dc->u.s_string.len) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, (dc->u.s_string.string1), (dc->u.s_string.len)); (dpi)->len += dc->u.s_string.len; } else d_print_append_buffer ((dpi), (dc->u.s_string.string1), (dc->u.s_string.len)); } while (0);
      return;
    case DEMANGLE_COMPONENT_RESTRICT:
    case DEMANGLE_COMPONENT_VOLATILE:
    case DEMANGLE_COMPONENT_CONST:
      {
 struct d_print_mod *pdpm;
 for (pdpm = dpi->modifiers; pdpm != ((void *)0); pdpm = pdpm->next)
   {
     if (! pdpm->printed)
       {
  if (pdpm->mod->type != DEMANGLE_COMPONENT_RESTRICT
      && pdpm->mod->type != DEMANGLE_COMPONENT_VOLATILE
      && pdpm->mod->type != DEMANGLE_COMPONENT_CONST)
    break;
  if (pdpm->mod->type == dc->type)
    {
      d_print_comp (dpi, ((dc)->u.s_binary.left));
      return;
    }
       }
   }
      }
    case DEMANGLE_COMPONENT_RESTRICT_THIS:
    case DEMANGLE_COMPONENT_VOLATILE_THIS:
    case DEMANGLE_COMPONENT_CONST_THIS:
    case DEMANGLE_COMPONENT_VENDOR_TYPE_QUAL:
    case DEMANGLE_COMPONENT_POINTER:
    case DEMANGLE_COMPONENT_REFERENCE:
    case DEMANGLE_COMPONENT_COMPLEX:
    case DEMANGLE_COMPONENT_IMAGINARY:
      {
 struct d_print_mod dpm;
 dpm.next = dpi->modifiers;
 dpi->modifiers = &dpm;
 dpm.mod = dc;
 dpm.printed = 0;
 dpm.templates = dpi->templates;
 d_print_comp (dpi, ((dc)->u.s_binary.left));
 if (! dpm.printed)
   d_print_mod (dpi, dc);
 dpi->modifiers = dpm.next;
 return;
      }
    case DEMANGLE_COMPONENT_BUILTIN_TYPE:
      if ((dpi->options & (1 << 2)) == 0)
 do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (dc->u.s_builtin.type->len) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, (dc->u.s_builtin.type->name), (dc->u.s_builtin.type->len)); (dpi)->len += dc->u.s_builtin.type->len; } else d_print_append_buffer ((dpi), (dc->u.s_builtin.type->name), (dc->u.s_builtin.type->len)); } while (0)
                              ;
      else
 do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (dc->u.s_builtin.type->java_len) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, (dc->u.s_builtin.type->java_name), (dc->u.s_builtin.type->java_len)); (dpi)->len += dc->u.s_builtin.type->java_len; } else d_print_append_buffer ((dpi), (dc->u.s_builtin.type->java_name), (dc->u.s_builtin.type->java_len)); } while (0)
                                   ;
      return;
    case DEMANGLE_COMPONENT_VENDOR_TYPE:
      d_print_comp (dpi, ((dc)->u.s_binary.left));
      return;
    case DEMANGLE_COMPONENT_FUNCTION_TYPE:
      {
 if (((dc)->u.s_binary.left) != ((void *)0))
   {
     struct d_print_mod dpm;
     dpm.next = dpi->modifiers;
     dpi->modifiers = &dpm;
     dpm.mod = dc;
     dpm.printed = 0;
     dpm.templates = dpi->templates;
     d_print_comp (dpi, ((dc)->u.s_binary.left));
     dpi->modifiers = dpm.next;
     if (dpm.printed)
       return;
     do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = (' '); else d_print_append_char ((dpi), (' ')); } while (0);
   }
 d_print_function_type (dpi, dc, dpi->modifiers);
 return;
      }
    case DEMANGLE_COMPONENT_ARRAY_TYPE:
      {
 struct d_print_mod *hold_modifiers;
 struct d_print_mod adpm[4];
 unsigned int i;
 struct d_print_mod *pdpm;
 hold_modifiers = dpi->modifiers;
 adpm[0].next = hold_modifiers;
 dpi->modifiers = &adpm[0];
 adpm[0].mod = dc;
 adpm[0].printed = 0;
 adpm[0].templates = dpi->templates;
 i = 1;
 pdpm = hold_modifiers;
 while (pdpm != ((void *)0)
        && (pdpm->mod->type == DEMANGLE_COMPONENT_RESTRICT
     || pdpm->mod->type == DEMANGLE_COMPONENT_VOLATILE
     || pdpm->mod->type == DEMANGLE_COMPONENT_CONST))
   {
     if (! pdpm->printed)
       {
  if (i >= sizeof adpm / sizeof adpm[0])
    {
      d_print_error (dpi);
      return;
    }
  adpm[i] = *pdpm;
  adpm[i].next = dpi->modifiers;
  dpi->modifiers = &adpm[i];
  pdpm->printed = 1;
  ++i;
       }
     pdpm = pdpm->next;
   }
 d_print_comp (dpi, ((dc)->u.s_binary.right));
 dpi->modifiers = hold_modifiers;
 if (adpm[0].printed)
   return;
 while (i > 1)
   {
     --i;
     d_print_mod (dpi, adpm[i].mod);
   }
 d_print_array_type (dpi, dc, dpi->modifiers);
 return;
      }
    case DEMANGLE_COMPONENT_PTRMEM_TYPE:
      {
 struct d_print_mod dpm;
 dpm.next = dpi->modifiers;
 dpi->modifiers = &dpm;
 dpm.mod = dc;
 dpm.printed = 0;
 dpm.templates = dpi->templates;
 d_print_comp (dpi, ((dc)->u.s_binary.right));
 if (! dpm.printed)
   {
     do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = (' '); else d_print_append_char ((dpi), (' ')); } while (0);
     d_print_comp (dpi, ((dc)->u.s_binary.left));
     do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (sizeof ("::*") - 1) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, (("::*")), (sizeof ("::*") - 1)); (dpi)->len += sizeof ("::*") - 1; } else d_print_append_buffer ((dpi), (("::*")), (sizeof ("::*") - 1)); } while (0);
   }
 dpi->modifiers = dpm.next;
 return;
      }
    case DEMANGLE_COMPONENT_ARGLIST:
    case DEMANGLE_COMPONENT_TEMPLATE_ARGLIST:
      d_print_comp (dpi, ((dc)->u.s_binary.left));
      if (((dc)->u.s_binary.right) != ((void *)0))
 {
   do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (sizeof (", ") - 1) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, ((", ")), (sizeof (", ") - 1)); (dpi)->len += sizeof (", ") - 1; } else d_print_append_buffer ((dpi), ((", ")), (sizeof (", ") - 1)); } while (0);
   d_print_comp (dpi, ((dc)->u.s_binary.right));
 }
      return;
    case DEMANGLE_COMPONENT_OPERATOR:
      {
 char c;
 do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (sizeof ("operator") - 1) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, (("operator")), (sizeof ("operator") - 1)); (dpi)->len += sizeof ("operator") - 1; } else d_print_append_buffer ((dpi), (("operator")), (sizeof ("operator") - 1)); } while (0);
 c = dc->u.s_operator.op->name[0];
 if (((c) >= 'a' && (c) <= 'z'))
   do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = (' '); else d_print_append_char ((dpi), (' ')); } while (0);
 do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (dc->u.s_operator.op->len) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, (dc->u.s_operator.op->name), (dc->u.s_operator.op->len)); (dpi)->len += dc->u.s_operator.op->len; } else d_print_append_buffer ((dpi), (dc->u.s_operator.op->name), (dc->u.s_operator.op->len)); } while (0)
                             ;
 return;
      }
    case DEMANGLE_COMPONENT_EXTENDED_OPERATOR:
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (sizeof ("operator ") - 1) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, (("operator ")), (sizeof ("operator ") - 1)); (dpi)->len += sizeof ("operator ") - 1; } else d_print_append_buffer ((dpi), (("operator ")), (sizeof ("operator ") - 1)); } while (0);
      d_print_comp (dpi, dc->u.s_extended_operator.name);
      return;
    case DEMANGLE_COMPONENT_CAST:
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (sizeof ("operator ") - 1) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, (("operator ")), (sizeof ("operator ") - 1)); (dpi)->len += sizeof ("operator ") - 1; } else d_print_append_buffer ((dpi), (("operator ")), (sizeof ("operator ") - 1)); } while (0);
      d_print_cast (dpi, dc);
      return;
    case DEMANGLE_COMPONENT_UNARY:
      if (((dc)->u.s_binary.left)->type != DEMANGLE_COMPONENT_CAST)
 d_print_expr_op (dpi, ((dc)->u.s_binary.left));
      else
 {
   do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = ('('); else d_print_append_char ((dpi), ('(')); } while (0);
   d_print_cast (dpi, ((dc)->u.s_binary.left));
   do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = (')'); else d_print_append_char ((dpi), (')')); } while (0);
 }
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = ('('); else d_print_append_char ((dpi), ('(')); } while (0);
      d_print_comp (dpi, ((dc)->u.s_binary.right));
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = (')'); else d_print_append_char ((dpi), (')')); } while (0);
      return;
    case DEMANGLE_COMPONENT_BINARY:
      if (((dc)->u.s_binary.right)->type != DEMANGLE_COMPONENT_BINARY_ARGS)
 {
   d_print_error (dpi);
   return;
 }
      if (((dc)->u.s_binary.left)->type == DEMANGLE_COMPONENT_OPERATOR
   && ((dc)->u.s_binary.left)->u.s_operator.op->len == 1
   && ((dc)->u.s_binary.left)->u.s_operator.op->name[0] == '>')
 do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = ('('); else d_print_append_char ((dpi), ('(')); } while (0);
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = ('('); else d_print_append_char ((dpi), ('(')); } while (0);
      d_print_comp (dpi, ((((dc)->u.s_binary.right))->u.s_binary.left));
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (sizeof (") ") - 1) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, ((") ")), (sizeof (") ") - 1)); (dpi)->len += sizeof (") ") - 1; } else d_print_append_buffer ((dpi), ((") ")), (sizeof (") ") - 1)); } while (0);
      d_print_expr_op (dpi, ((dc)->u.s_binary.left));
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (sizeof (" (") - 1) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, ((" (")), (sizeof (" (") - 1)); (dpi)->len += sizeof (" (") - 1; } else d_print_append_buffer ((dpi), ((" (")), (sizeof (" (") - 1)); } while (0);
      d_print_comp (dpi, ((((dc)->u.s_binary.right))->u.s_binary.right));
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = (')'); else d_print_append_char ((dpi), (')')); } while (0);
      if (((dc)->u.s_binary.left)->type == DEMANGLE_COMPONENT_OPERATOR
   && ((dc)->u.s_binary.left)->u.s_operator.op->len == 1
   && ((dc)->u.s_binary.left)->u.s_operator.op->name[0] == '>')
 do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = (')'); else d_print_append_char ((dpi), (')')); } while (0);
      return;
    case DEMANGLE_COMPONENT_BINARY_ARGS:
      d_print_error (dpi);
      return;
    case DEMANGLE_COMPONENT_TRINARY:
      if (((dc)->u.s_binary.right)->type != DEMANGLE_COMPONENT_TRINARY_ARG1
   || ((((dc)->u.s_binary.right))->u.s_binary.right)->type != DEMANGLE_COMPONENT_TRINARY_ARG2)
 {
   d_print_error (dpi);
   return;
 }
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = ('('); else d_print_append_char ((dpi), ('(')); } while (0);
      d_print_comp (dpi, ((((dc)->u.s_binary.right))->u.s_binary.left));
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (sizeof (") ") - 1) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, ((") ")), (sizeof (") ") - 1)); (dpi)->len += sizeof (") ") - 1; } else d_print_append_buffer ((dpi), ((") ")), (sizeof (") ") - 1)); } while (0);
      d_print_expr_op (dpi, ((dc)->u.s_binary.left));
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (sizeof (" (") - 1) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, ((" (")), (sizeof (" (") - 1)); (dpi)->len += sizeof (" (") - 1; } else d_print_append_buffer ((dpi), ((" (")), (sizeof (" (") - 1)); } while (0);
      d_print_comp (dpi, ((((((dc)->u.s_binary.right))->u.s_binary.right))->u.s_binary.left));
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (sizeof (") : (") - 1) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, ((") : (")), (sizeof (") : (") - 1)); (dpi)->len += sizeof (") : (") - 1; } else d_print_append_buffer ((dpi), ((") : (")), (sizeof (") : (") - 1)); } while (0);
      d_print_comp (dpi, ((((((dc)->u.s_binary.right))->u.s_binary.right))->u.s_binary.right));
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = (')'); else d_print_append_char ((dpi), (')')); } while (0);
      return;
    case DEMANGLE_COMPONENT_TRINARY_ARG1:
    case DEMANGLE_COMPONENT_TRINARY_ARG2:
      d_print_error (dpi);
      return;
    case DEMANGLE_COMPONENT_LITERAL:
    case DEMANGLE_COMPONENT_LITERAL_NEG:
      {
 enum d_builtin_type_print tp;
 tp = D_PRINT_DEFAULT;
 if (((dc)->u.s_binary.left)->type == DEMANGLE_COMPONENT_BUILTIN_TYPE)
   {
     tp = ((dc)->u.s_binary.left)->u.s_builtin.type->print;
     switch (tp)
       {
       case D_PRINT_INT:
       case D_PRINT_UNSIGNED:
       case D_PRINT_LONG:
       case D_PRINT_UNSIGNED_LONG:
       case D_PRINT_LONG_LONG:
       case D_PRINT_UNSIGNED_LONG_LONG:
  if (((dc)->u.s_binary.right)->type == DEMANGLE_COMPONENT_NAME)
    {
      if (dc->type == DEMANGLE_COMPONENT_LITERAL_NEG)
        do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = ('-'); else d_print_append_char ((dpi), ('-')); } while (0);
      d_print_comp (dpi, ((dc)->u.s_binary.right));
      switch (tp)
        {
        default:
   break;
        case D_PRINT_UNSIGNED:
   do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = ('u'); else d_print_append_char ((dpi), ('u')); } while (0);
   break;
        case D_PRINT_LONG:
   do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = ('l'); else d_print_append_char ((dpi), ('l')); } while (0);
   break;
        case D_PRINT_UNSIGNED_LONG:
   do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (sizeof ("ul") - 1) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, (("ul")), (sizeof ("ul") - 1)); (dpi)->len += sizeof ("ul") - 1; } else d_print_append_buffer ((dpi), (("ul")), (sizeof ("ul") - 1)); } while (0);
   break;
        case D_PRINT_LONG_LONG:
   do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (sizeof ("ll") - 1) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, (("ll")), (sizeof ("ll") - 1)); (dpi)->len += sizeof ("ll") - 1; } else d_print_append_buffer ((dpi), (("ll")), (sizeof ("ll") - 1)); } while (0);
   break;
        case D_PRINT_UNSIGNED_LONG_LONG:
   do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (sizeof ("ull") - 1) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, (("ull")), (sizeof ("ull") - 1)); (dpi)->len += sizeof ("ull") - 1; } else d_print_append_buffer ((dpi), (("ull")), (sizeof ("ull") - 1)); } while (0);
   break;
        }
      return;
    }
  break;
       case D_PRINT_BOOL:
  if (((dc)->u.s_binary.right)->type == DEMANGLE_COMPONENT_NAME
      && ((dc)->u.s_binary.right)->u.s_name.len == 1
      && dc->type == DEMANGLE_COMPONENT_LITERAL)
    {
      switch (((dc)->u.s_binary.right)->u.s_name.s[0])
        {
        case '0':
   do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (sizeof ("false") - 1) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, (("false")), (sizeof ("false") - 1)); (dpi)->len += sizeof ("false") - 1; } else d_print_append_buffer ((dpi), (("false")), (sizeof ("false") - 1)); } while (0);
   return;
        case '1':
   do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (sizeof ("true") - 1) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, (("true")), (sizeof ("true") - 1)); (dpi)->len += sizeof ("true") - 1; } else d_print_append_buffer ((dpi), (("true")), (sizeof ("true") - 1)); } while (0);
   return;
        default:
   break;
        }
    }
  break;
       default:
  break;
       }
   }
 do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = ('('); else d_print_append_char ((dpi), ('(')); } while (0);
 d_print_comp (dpi, ((dc)->u.s_binary.left));
 do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = (')'); else d_print_append_char ((dpi), (')')); } while (0);
 if (dc->type == DEMANGLE_COMPONENT_LITERAL_NEG)
   do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = ('-'); else d_print_append_char ((dpi), ('-')); } while (0);
 if (tp == D_PRINT_FLOAT)
   do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = ('['); else d_print_append_char ((dpi), ('[')); } while (0);
 d_print_comp (dpi, ((dc)->u.s_binary.right));
 if (tp == D_PRINT_FLOAT)
   do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = (']'); else d_print_append_char ((dpi), (']')); } while (0);
      }
      return;
    default:
      d_print_error (dpi);
      return;
    }
}
static void
d_print_java_identifier (dpi, name, len)
     struct d_print_info *dpi;
     const char *name;
     int len;
{
  const char *p;
  const char *end;
  end = name + len;
  for (p = name; p < end; ++p)
    {
      if (end - p > 3
   && p[0] == '_'
   && p[1] == '_'
   && p[2] == 'U')
 {
   unsigned long c;
   const char *q;
   c = 0;
   for (q = p + 3; q < end; ++q)
     {
       int dig;
       if (((*q) >= '0' && (*q) <= '9'))
  dig = *q - '0';
       else if (*q >= 'A' && *q <= 'F')
  dig = *q - 'A' + 10;
       else if (*q >= 'a' && *q <= 'f')
  dig = *q - 'a' + 10;
       else
  break;
       c = c * 16 + dig;
     }
   if (q < end && *q == '_' && c < 256)
     {
       do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = (c); else d_print_append_char ((dpi), (c)); } while (0);
       p = q;
       continue;
     }
 }
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = (*p); else d_print_append_char ((dpi), (*p)); } while (0);
    }
}
static void
d_print_mod_list (dpi, mods, suffix)
     struct d_print_info *dpi;
     struct d_print_mod *mods;
     int suffix;
{
  struct d_print_template *hold_dpt;
  if (mods == ((void *)0) || ((dpi)->buf == ((void *)0)))
    return;
  if (mods->printed
      || (! suffix
   && (mods->mod->type == DEMANGLE_COMPONENT_RESTRICT_THIS
       || mods->mod->type == DEMANGLE_COMPONENT_VOLATILE_THIS
       || mods->mod->type == DEMANGLE_COMPONENT_CONST_THIS)))
    {
      d_print_mod_list (dpi, mods->next, suffix);
      return;
    }
  mods->printed = 1;
  hold_dpt = dpi->templates;
  dpi->templates = mods->templates;
  if (mods->mod->type == DEMANGLE_COMPONENT_FUNCTION_TYPE)
    {
      d_print_function_type (dpi, mods->mod, mods->next);
      dpi->templates = hold_dpt;
      return;
    }
  else if (mods->mod->type == DEMANGLE_COMPONENT_ARRAY_TYPE)
    {
      d_print_array_type (dpi, mods->mod, mods->next);
      dpi->templates = hold_dpt;
      return;
    }
  else if (mods->mod->type == DEMANGLE_COMPONENT_LOCAL_NAME)
    {
      struct d_print_mod *hold_modifiers;
      struct demangle_component *dc;
      hold_modifiers = dpi->modifiers;
      dpi->modifiers = ((void *)0);
      d_print_comp (dpi, ((mods->mod)->u.s_binary.left));
      dpi->modifiers = hold_modifiers;
      if ((dpi->options & (1 << 2)) == 0)
 do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (sizeof ("::") - 1) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, (("::")), (sizeof ("::") - 1)); (dpi)->len += sizeof ("::") - 1; } else d_print_append_buffer ((dpi), (("::")), (sizeof ("::") - 1)); } while (0);
      else
 do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = ('.'); else d_print_append_char ((dpi), ('.')); } while (0);
      dc = ((mods->mod)->u.s_binary.right);
      while (dc->type == DEMANGLE_COMPONENT_RESTRICT_THIS
      || dc->type == DEMANGLE_COMPONENT_VOLATILE_THIS
      || dc->type == DEMANGLE_COMPONENT_CONST_THIS)
 dc = ((dc)->u.s_binary.left);
      d_print_comp (dpi, dc);
      dpi->templates = hold_dpt;
      return;
    }
  d_print_mod (dpi, mods->mod);
  dpi->templates = hold_dpt;
  d_print_mod_list (dpi, mods->next, suffix);
}
static void
d_print_mod (dpi, mod)
     struct d_print_info *dpi;
     const struct demangle_component *mod;
{
  switch (mod->type)
    {
    case DEMANGLE_COMPONENT_RESTRICT:
    case DEMANGLE_COMPONENT_RESTRICT_THIS:
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (sizeof (" restrict") - 1) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, ((" restrict")), (sizeof (" restrict") - 1)); (dpi)->len += sizeof (" restrict") - 1; } else d_print_append_buffer ((dpi), ((" restrict")), (sizeof (" restrict") - 1)); } while (0);
      return;
    case DEMANGLE_COMPONENT_VOLATILE:
    case DEMANGLE_COMPONENT_VOLATILE_THIS:
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (sizeof (" volatile") - 1) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, ((" volatile")), (sizeof (" volatile") - 1)); (dpi)->len += sizeof (" volatile") - 1; } else d_print_append_buffer ((dpi), ((" volatile")), (sizeof (" volatile") - 1)); } while (0);
      return;
    case DEMANGLE_COMPONENT_CONST:
    case DEMANGLE_COMPONENT_CONST_THIS:
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (sizeof (" const") - 1) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, ((" const")), (sizeof (" const") - 1)); (dpi)->len += sizeof (" const") - 1; } else d_print_append_buffer ((dpi), ((" const")), (sizeof (" const") - 1)); } while (0);
      return;
    case DEMANGLE_COMPONENT_VENDOR_TYPE_QUAL:
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = (' '); else d_print_append_char ((dpi), (' ')); } while (0);
      d_print_comp (dpi, ((mod)->u.s_binary.right));
      return;
    case DEMANGLE_COMPONENT_POINTER:
      if ((dpi->options & (1 << 2)) == 0)
 do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = ('*'); else d_print_append_char ((dpi), ('*')); } while (0);
      return;
    case DEMANGLE_COMPONENT_REFERENCE:
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = ('&'); else d_print_append_char ((dpi), ('&')); } while (0);
      return;
    case DEMANGLE_COMPONENT_COMPLEX:
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (sizeof ("complex ") - 1) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, (("complex ")), (sizeof ("complex ") - 1)); (dpi)->len += sizeof ("complex ") - 1; } else d_print_append_buffer ((dpi), (("complex ")), (sizeof ("complex ") - 1)); } while (0);
      return;
    case DEMANGLE_COMPONENT_IMAGINARY:
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (sizeof ("imaginary ") - 1) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, (("imaginary ")), (sizeof ("imaginary ") - 1)); (dpi)->len += sizeof ("imaginary ") - 1; } else d_print_append_buffer ((dpi), (("imaginary ")), (sizeof ("imaginary ") - 1)); } while (0);
      return;
    case DEMANGLE_COMPONENT_PTRMEM_TYPE:
      if (((dpi)->buf == ((void *)0) || (dpi)->len == 0 ? 0 : (dpi)->buf[(dpi)->len - 1]) != '(')
 do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = (' '); else d_print_append_char ((dpi), (' ')); } while (0);
      d_print_comp (dpi, ((mod)->u.s_binary.left));
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (sizeof ("::*") - 1) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, (("::*")), (sizeof ("::*") - 1)); (dpi)->len += sizeof ("::*") - 1; } else d_print_append_buffer ((dpi), (("::*")), (sizeof ("::*") - 1)); } while (0);
      return;
    case DEMANGLE_COMPONENT_TYPED_NAME:
      d_print_comp (dpi, ((mod)->u.s_binary.left));
      return;
    default:
      d_print_comp (dpi, mod);
      return;
    }
}
static void
d_print_function_type (dpi, dc, mods)
     struct d_print_info *dpi;
     const struct demangle_component *dc;
     struct d_print_mod *mods;
{
  int need_paren;
  int saw_mod;
  int need_space;
  struct d_print_mod *p;
  struct d_print_mod *hold_modifiers;
  need_paren = 0;
  saw_mod = 0;
  need_space = 0;
  for (p = mods; p != ((void *)0); p = p->next)
    {
      if (p->printed)
 break;
      saw_mod = 1;
      switch (p->mod->type)
 {
 case DEMANGLE_COMPONENT_POINTER:
 case DEMANGLE_COMPONENT_REFERENCE:
   need_paren = 1;
   break;
 case DEMANGLE_COMPONENT_RESTRICT:
 case DEMANGLE_COMPONENT_VOLATILE:
 case DEMANGLE_COMPONENT_CONST:
 case DEMANGLE_COMPONENT_VENDOR_TYPE_QUAL:
 case DEMANGLE_COMPONENT_COMPLEX:
 case DEMANGLE_COMPONENT_IMAGINARY:
 case DEMANGLE_COMPONENT_PTRMEM_TYPE:
   need_space = 1;
   need_paren = 1;
   break;
 case DEMANGLE_COMPONENT_RESTRICT_THIS:
 case DEMANGLE_COMPONENT_VOLATILE_THIS:
 case DEMANGLE_COMPONENT_CONST_THIS:
   break;
 default:
   break;
 }
      if (need_paren)
 break;
    }
  if (((dc)->u.s_binary.left) != ((void *)0) && ! saw_mod)
    need_paren = 1;
  if (need_paren)
    {
      if (! need_space)
 {
   if (((dpi)->buf == ((void *)0) || (dpi)->len == 0 ? 0 : (dpi)->buf[(dpi)->len - 1]) != '('
       && ((dpi)->buf == ((void *)0) || (dpi)->len == 0 ? 0 : (dpi)->buf[(dpi)->len - 1]) != '*')
     need_space = 1;
 }
      if (need_space && ((dpi)->buf == ((void *)0) || (dpi)->len == 0 ? 0 : (dpi)->buf[(dpi)->len - 1]) != ' ')
 do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = (' '); else d_print_append_char ((dpi), (' ')); } while (0);
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = ('('); else d_print_append_char ((dpi), ('(')); } while (0);
    }
  hold_modifiers = dpi->modifiers;
  dpi->modifiers = ((void *)0);
  d_print_mod_list (dpi, mods, 0);
  if (need_paren)
    do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = (')'); else d_print_append_char ((dpi), (')')); } while (0);
  do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = ('('); else d_print_append_char ((dpi), ('(')); } while (0);
  if (((dc)->u.s_binary.right) != ((void *)0))
    d_print_comp (dpi, ((dc)->u.s_binary.right));
  do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = (')'); else d_print_append_char ((dpi), (')')); } while (0);
  d_print_mod_list (dpi, mods, 1);
  dpi->modifiers = hold_modifiers;
}
static void
d_print_array_type (dpi, dc, mods)
     struct d_print_info *dpi;
     const struct demangle_component *dc;
     struct d_print_mod *mods;
{
  int need_space;
  need_space = 1;
  if (mods != ((void *)0))
    {
      int need_paren;
      struct d_print_mod *p;
      need_paren = 0;
      for (p = mods; p != ((void *)0); p = p->next)
 {
   if (! p->printed)
     {
       if (p->mod->type == DEMANGLE_COMPONENT_ARRAY_TYPE)
  {
    need_space = 0;
    break;
  }
       else
  {
    need_paren = 1;
    need_space = 1;
    break;
  }
     }
 }
      if (need_paren)
 do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (sizeof (" (") - 1) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, ((" (")), (sizeof (" (") - 1)); (dpi)->len += sizeof (" (") - 1; } else d_print_append_buffer ((dpi), ((" (")), (sizeof (" (") - 1)); } while (0);
      d_print_mod_list (dpi, mods, 0);
      if (need_paren)
 do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = (')'); else d_print_append_char ((dpi), (')')); } while (0);
    }
  if (need_space)
    do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = (' '); else d_print_append_char ((dpi), (' ')); } while (0);
  do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = ('['); else d_print_append_char ((dpi), ('[')); } while (0);
  if (((dc)->u.s_binary.left) != ((void *)0))
    d_print_comp (dpi, ((dc)->u.s_binary.left));
  do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = (']'); else d_print_append_char ((dpi), (']')); } while (0);
}
static void
d_print_expr_op (dpi, dc)
     struct d_print_info *dpi;
     const struct demangle_component *dc;
{
  if (dc->type == DEMANGLE_COMPONENT_OPERATOR)
    do { if ((dpi)->buf != ((void *)0) && (dpi)->len + (dc->u.s_operator.op->len) <= (dpi)->alc) { memcpy ((dpi)->buf + (dpi)->len, (dc->u.s_operator.op->name), (dc->u.s_operator.op->len)); (dpi)->len += dc->u.s_operator.op->len; } else d_print_append_buffer ((dpi), (dc->u.s_operator.op->name), (dc->u.s_operator.op->len)); } while (0)
                                ;
  else
    d_print_comp (dpi, dc);
}
static void
d_print_cast (dpi, dc)
     struct d_print_info *dpi;
     const struct demangle_component *dc;
{
  if (((dc)->u.s_binary.left)->type != DEMANGLE_COMPONENT_TEMPLATE)
    d_print_comp (dpi, ((dc)->u.s_binary.left));
  else
    {
      struct d_print_mod *hold_dpm;
      struct d_print_template dpt;
      hold_dpm = dpi->modifiers;
      dpi->modifiers = ((void *)0);
      dpt.next = dpi->templates;
      dpi->templates = &dpt;
      dpt.template = ((dc)->u.s_binary.left);
      d_print_comp (dpi, ((((dc)->u.s_binary.left))->u.s_binary.left));
      dpi->templates = dpt.next;
      if (((dpi)->buf == ((void *)0) || (dpi)->len == 0 ? 0 : (dpi)->buf[(dpi)->len - 1]) == '<')
 do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = (' '); else d_print_append_char ((dpi), (' ')); } while (0);
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = ('<'); else d_print_append_char ((dpi), ('<')); } while (0);
      d_print_comp (dpi, ((((dc)->u.s_binary.left))->u.s_binary.right));
      if (((dpi)->buf == ((void *)0) || (dpi)->len == 0 ? 0 : (dpi)->buf[(dpi)->len - 1]) == '>')
 do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = (' '); else d_print_append_char ((dpi), (' ')); } while (0);
      do { if ((dpi)->buf != ((void *)0) && (dpi)->len < (dpi)->alc) (dpi)->buf[(dpi)->len++] = ('>'); else d_print_append_char ((dpi), ('>')); } while (0);
      dpi->modifiers = hold_dpm;
    }
}
void
cplus_demangle_init_info (mangled, options, len, di)
     const char *mangled;
     int options;
     size_t len;
     struct d_info *di;
{
  di->s = mangled;
  di->send = mangled + len;
  di->options = options;
  di->n = mangled;
  di->num_comps = 2 * len;
  di->next_comp = 0;
  di->num_subs = len;
  di->next_sub = 0;
  di->did_subs = 0;
  di->last_name = ((void *)0);
  di->expansion = 0;
}
static char *
d_demangle (mangled, options, palc)
     const char* mangled;
     int options;
     size_t *palc;
{
  size_t len;
  int type;
  struct d_info di;
  struct demangle_component *dc;
  int estimate;
  char *ret;
  *palc = 0;
  len = strlen (mangled);
  if (mangled[0] == '_' && mangled[1] == 'Z')
    type = 0;
  else if (strncmp (mangled, "_GLOBAL_", 8) == 0
    && (mangled[8] == '.' || mangled[8] == '_' || mangled[8] == '$')
    && (mangled[9] == 'D' || mangled[9] == 'I')
    && mangled[10] == '_')
    {
      char *r;
      r = malloc (40 + len - 11);
      if (r == ((void *)0))
 *palc = 1;
      else
 {
   if (mangled[9] == 'I')
     strcpy (r, "global constructors keyed to ");
   else
     strcpy (r, "global destructors keyed to ");
   strcat (r, mangled + 11);
 }
      return r;
    }
  else
    {
      if ((options & (1 << 4)) == 0)
 return ((void *)0);
      type = 1;
    }
  cplus_demangle_init_info (mangled, options, len, &di);
  {
    di.comps = ((struct demangle_component *)
  malloc (di.num_comps * sizeof (struct demangle_component)));
    di.subs = ((struct demangle_component *)
        malloc (di.num_subs * sizeof (struct demangle_component *)));
    if (di.comps == ((void *)0) || di.subs == ((void *)0))
      {
 if (di.comps != ((void *)0))
   free (di.comps);
 if (di.subs != ((void *)0))
   free (di.subs);
 *palc = 1;
 return ((void *)0);
      }
    if (! type)
      dc = cplus_demangle_mangled_name (&di, 1);
    else
      dc = cplus_demangle_type (&di);
    if (((options & (1 << 0)) != 0) && (*((&di)->n)) != 0)
      dc = ((void *)0);
    estimate = len + di.expansion + 10 * di.did_subs;
    estimate += estimate / 8;
    ret = ((void *)0);
    if (dc != ((void *)0))
      ret = cplus_demangle_print (options, dc, estimate, palc);
    free (di.comps);
    free (di.subs);
  }
  return ret;
}
char *
cplus_demangle_v3 (mangled, options)
     const char* mangled;
     int options;
{
  size_t alc;
  return d_demangle (mangled, options, &alc);
}
char *
java_demangle_v3 (mangled)
     const char* mangled;
{
  size_t alc;
  char *demangled;
  int nesting;
  char *from;
  char *to;
  demangled = d_demangle (mangled, (1 << 2) | (1 << 0), &alc);
  if (demangled == ((void *)0))
    return ((void *)0);
  nesting = 0;
  from = demangled;
  to = from;
  while (*from != 0)
    {
      if (strncmp (from, "JArray<", 7) == 0)
 {
   from += 7;
   ++nesting;
 }
      else if (nesting > 0 && *from == '>')
 {
   while (to > demangled && to[-1] == ' ')
     --to;
   *to++ = '[';
   *to++ = ']';
   --nesting;
   ++from;
 }
      else
 *to++ = *from++;
    }
  *to = 0;
  return demangled;
}
static int
is_ctor_or_dtor (mangled, ctor_kind, dtor_kind)
     const char *mangled;
     enum gnu_v3_ctor_kinds *ctor_kind;
     enum gnu_v3_dtor_kinds *dtor_kind;
{
  struct d_info di;
  struct demangle_component *dc;
  int ret;
  *ctor_kind = (enum gnu_v3_ctor_kinds) 0;
  *dtor_kind = (enum gnu_v3_dtor_kinds) 0;
  cplus_demangle_init_info (mangled, (1 << 14), strlen (mangled), &di);
  {
    di.comps = ((struct demangle_component *)
  malloc (di.num_comps * sizeof (struct demangle_component)));
    di.subs = ((struct demangle_component *)
        malloc (di.num_subs * sizeof (struct demangle_component *)));
    if (di.comps == ((void *)0) || di.subs == ((void *)0))
      {
 if (di.comps != ((void *)0))
   free (di.comps);
 if (di.subs != ((void *)0))
   free (di.subs);
 return 0;
      }
    dc = cplus_demangle_mangled_name (&di, 1);
    ret = 0;
    while (dc != ((void *)0))
      {
 switch (dc->type)
   {
   default:
     dc = ((void *)0);
     break;
   case DEMANGLE_COMPONENT_TYPED_NAME:
   case DEMANGLE_COMPONENT_TEMPLATE:
   case DEMANGLE_COMPONENT_RESTRICT_THIS:
   case DEMANGLE_COMPONENT_VOLATILE_THIS:
   case DEMANGLE_COMPONENT_CONST_THIS:
     dc = ((dc)->u.s_binary.left);
     break;
   case DEMANGLE_COMPONENT_QUAL_NAME:
   case DEMANGLE_COMPONENT_LOCAL_NAME:
     dc = ((dc)->u.s_binary.right);
     break;
   case DEMANGLE_COMPONENT_CTOR:
     *ctor_kind = dc->u.s_ctor.kind;
     ret = 1;
     dc = ((void *)0);
     break;
   case DEMANGLE_COMPONENT_DTOR:
     *dtor_kind = dc->u.s_dtor.kind;
     ret = 1;
     dc = ((void *)0);
     break;
   }
      }
    free (di.subs);
    free (di.comps);
  }
  return ret;
}
enum gnu_v3_ctor_kinds
is_gnu_v3_mangled_ctor (name)
     const char *name;
{
  enum gnu_v3_ctor_kinds ctor_kind;
  enum gnu_v3_dtor_kinds dtor_kind;
  if (! is_ctor_or_dtor (name, &ctor_kind, &dtor_kind))
    return (enum gnu_v3_ctor_kinds) 0;
  return ctor_kind;
}
enum gnu_v3_dtor_kinds
is_gnu_v3_mangled_dtor (name)
     const char *name;
{
  enum gnu_v3_ctor_kinds ctor_kind;
  enum gnu_v3_dtor_kinds dtor_kind;
  if (! is_ctor_or_dtor (name, &ctor_kind, &dtor_kind))
    return (enum gnu_v3_dtor_kinds) 0;
  return dtor_kind;
}
extern char *optarg;
extern int optind;
extern int opterr;
extern int optopt;
struct option
{
  const char *name;
  int has_arg;
  int *flag;
  int val;
};
extern int getopt (int argc, char *const *argv, const char *shortopts);
extern int getopt_long (int argc, char *const *argv, const char *shortopts,
          const struct option *longopts, int *longind);
extern int getopt_long_only (int argc, char *const *argv,
        const char *shortopts,
               const struct option *longopts, int *longind);
extern int _getopt_internal (int argc, char *const *argv,
        const char *shortopts,
               const struct option *longopts, int *longind,
        int long_only);
typedef struct dyn_string
{
  int allocated;
  int length;
  char *s;
}* dyn_string_t;
extern int dyn_string_init (struct dyn_string *, int);
extern dyn_string_t dyn_string_new (int);
extern void dyn_string_delete (dyn_string_t);
extern char *dyn_string_release (dyn_string_t);
extern dyn_string_t dyn_string_resize (dyn_string_t, int);
extern void dyn_string_clear (dyn_string_t);
extern int dyn_string_copy (dyn_string_t, dyn_string_t);
extern int dyn_string_copy_cstr (dyn_string_t, const char *);
extern int dyn_string_prepend (dyn_string_t, dyn_string_t);
extern int dyn_string_prepend_cstr (dyn_string_t, const char *);
extern int dyn_string_insert (dyn_string_t, int, dyn_string_t)
                     ;
extern int dyn_string_insert_cstr (dyn_string_t, int, const char *)
                     ;
extern int dyn_string_insert_char (dyn_string_t, int, int);
extern int dyn_string_append (dyn_string_t, dyn_string_t);
extern int dyn_string_append_cstr (dyn_string_t, const char *);
extern int dyn_string_append_char (dyn_string_t, int);
extern int dyn_string_substring (dyn_string_t, dyn_string_t, int, int)
                               ;
extern int dyn_string_eq (dyn_string_t, dyn_string_t);
   typedef unsigned int md5_uint32;
struct md5_ctx
{
  md5_uint32 A;
  md5_uint32 B;
  md5_uint32 C;
  md5_uint32 D;
  md5_uint32 total[2];
  md5_uint32 buflen;
  char buffer[128];
};
extern void md5_init_ctx (struct md5_ctx *ctx);
extern void md5_process_block (const void *buffer, size_t len, struct md5_ctx *ctx)
                             ;
extern void md5_process_bytes (const void *buffer, size_t len, struct md5_ctx *ctx)
                             ;
extern void *md5_finish_ctx (struct md5_ctx *ctx, void *resbuf);
extern void *md5_read_ctx (const struct md5_ctx *ctx, void *resbuf);
extern int md5_stream (FILE *stream, void *resblock);
extern void *md5_buffer (const char *buffer, size_t len, void *resblock);
static const unsigned char fillbuf[64] = { 0x80, 0 };
void
md5_init_ctx (ctx)
     struct md5_ctx *ctx;
{
  ctx->A = (md5_uint32) 0x67452301;
  ctx->B = (md5_uint32) 0xefcdab89;
  ctx->C = (md5_uint32) 0x98badcfe;
  ctx->D = (md5_uint32) 0x10325476;
  ctx->total[0] = ctx->total[1] = 0;
  ctx->buflen = 0;
}
void *
md5_read_ctx (ctx, resbuf)
     const struct md5_ctx *ctx;
     void *resbuf;
{
  ((md5_uint32 *) resbuf)[0] = (ctx->A);
  ((md5_uint32 *) resbuf)[1] = (ctx->B);
  ((md5_uint32 *) resbuf)[2] = (ctx->C);
  ((md5_uint32 *) resbuf)[3] = (ctx->D);
  return resbuf;
}
void *
md5_finish_ctx (ctx, resbuf)
     struct md5_ctx *ctx;
     void *resbuf;
{
  md5_uint32 bytes = ctx->buflen;
  size_t pad;
  ctx->total[0] += bytes;
  if (ctx->total[0] < bytes)
    ++ctx->total[1];
  pad = bytes >= 56 ? 64 + 56 - bytes : 56 - bytes;
  memcpy (&ctx->buffer[bytes], fillbuf, pad);
  *(md5_uint32 *) &ctx->buffer[bytes + pad] = (ctx->total[0] << 3);
  *(md5_uint32 *) &ctx->buffer[bytes + pad + 4] = ((ctx->total[1] << 3) | (ctx->total[0] >> 29))
                             ;
  md5_process_block (ctx->buffer, bytes + pad + 8, ctx);
  return md5_read_ctx (ctx, resbuf);
}
int
md5_stream (stream, resblock)
     FILE *stream;
     void *resblock;
{
  struct md5_ctx ctx;
  char buffer[4096 + 72];
  size_t sum;
  md5_init_ctx (&ctx);
  while (1)
    {
      size_t n;
      sum = 0;
      do
 {
   n = fread (buffer + sum, 1, 4096 - sum, stream);
   sum += n;
 }
      while (sum < 4096 && n != 0);
      if (n == 0 && ferror (stream))
        return 1;
      if (n == 0)
 break;
      md5_process_block (buffer, 4096, &ctx);
    }
  if (sum > 0)
    md5_process_bytes (buffer, sum, &ctx);
  md5_finish_ctx (&ctx, resblock);
  return 0;
}
void *
md5_buffer (buffer, len, resblock)
     const char *buffer;
     size_t len;
     void *resblock;
{
  struct md5_ctx ctx;
  md5_init_ctx (&ctx);
  md5_process_bytes (buffer, len, &ctx);
  return md5_finish_ctx (&ctx, resblock);
}
void
md5_process_bytes (buffer, len, ctx)
     const void *buffer;
     size_t len;
     struct md5_ctx *ctx;
{
  if (ctx->buflen != 0)
    {
      size_t left_over = ctx->buflen;
      size_t add = 128 - left_over > len ? len : 128 - left_over;
      memcpy (&ctx->buffer[left_over], buffer, add);
      ctx->buflen += add;
      if (left_over + add > 64)
 {
   md5_process_block (ctx->buffer, (left_over + add) & ~63, ctx);
   memcpy (ctx->buffer, &ctx->buffer[(left_over + add) & ~63],
    (left_over + add) & 63);
   ctx->buflen = (left_over + add) & 63;
 }
      buffer = (const void *) ((const char *) buffer + add);
      len -= add;
    }
  if (len > 64)
    {
      md5_process_block (buffer, len & ~63, ctx);
      buffer = (const void *) ((const char *) buffer + (len & ~63));
      len &= 63;
    }
  if (len > 0)
    {
      memcpy (ctx->buffer, buffer, len);
      ctx->buflen = len;
    }
}
void
md5_process_block (buffer, len, ctx)
     const void *buffer;
     size_t len;
     struct md5_ctx *ctx;
{
  md5_uint32 correct_words[16];
  const md5_uint32 *words = (const md5_uint32 *) buffer;
  size_t nwords = len / sizeof (md5_uint32);
  const md5_uint32 *endp = words + nwords;
  md5_uint32 A = ctx->A;
  md5_uint32 B = ctx->B;
  md5_uint32 C = ctx->C;
  md5_uint32 D = ctx->D;
  ctx->total[0] += len;
  if (ctx->total[0] < len)
    ++ctx->total[1];
  while (words < endp)
    {
      md5_uint32 *cwp = correct_words;
      md5_uint32 A_save = A;
      md5_uint32 B_save = B;
      md5_uint32 C_save = C;
      md5_uint32 D_save = D;
      do { A += (D ^ (B & (C ^ D))) + (*cwp++ = (*words)) + (md5_uint32) 0xd76aa478; ++words; (A = (A << 7) | (A >> (32 - 7))); A += B; } while (0);
      do { D += (C ^ (A & (B ^ C))) + (*cwp++ = (*words)) + (md5_uint32) 0xe8c7b756; ++words; (D = (D << 12) | (D >> (32 - 12))); D += A; } while (0);
      do { C += (B ^ (D & (A ^ B))) + (*cwp++ = (*words)) + (md5_uint32) 0x242070db; ++words; (C = (C << 17) | (C >> (32 - 17))); C += D; } while (0);
      do { B += (A ^ (C & (D ^ A))) + (*cwp++ = (*words)) + (md5_uint32) 0xc1bdceee; ++words; (B = (B << 22) | (B >> (32 - 22))); B += C; } while (0);
      do { A += (D ^ (B & (C ^ D))) + (*cwp++ = (*words)) + (md5_uint32) 0xf57c0faf; ++words; (A = (A << 7) | (A >> (32 - 7))); A += B; } while (0);
      do { D += (C ^ (A & (B ^ C))) + (*cwp++ = (*words)) + (md5_uint32) 0x4787c62a; ++words; (D = (D << 12) | (D >> (32 - 12))); D += A; } while (0);
      do { C += (B ^ (D & (A ^ B))) + (*cwp++ = (*words)) + (md5_uint32) 0xa8304613; ++words; (C = (C << 17) | (C >> (32 - 17))); C += D; } while (0);
      do { B += (A ^ (C & (D ^ A))) + (*cwp++ = (*words)) + (md5_uint32) 0xfd469501; ++words; (B = (B << 22) | (B >> (32 - 22))); B += C; } while (0);
      do { A += (D ^ (B & (C ^ D))) + (*cwp++ = (*words)) + (md5_uint32) 0x698098d8; ++words; (A = (A << 7) | (A >> (32 - 7))); A += B; } while (0);
      do { D += (C ^ (A & (B ^ C))) + (*cwp++ = (*words)) + (md5_uint32) 0x8b44f7af; ++words; (D = (D << 12) | (D >> (32 - 12))); D += A; } while (0);
      do { C += (B ^ (D & (A ^ B))) + (*cwp++ = (*words)) + (md5_uint32) 0xffff5bb1; ++words; (C = (C << 17) | (C >> (32 - 17))); C += D; } while (0);
      do { B += (A ^ (C & (D ^ A))) + (*cwp++ = (*words)) + (md5_uint32) 0x895cd7be; ++words; (B = (B << 22) | (B >> (32 - 22))); B += C; } while (0);
      do { A += (D ^ (B & (C ^ D))) + (*cwp++ = (*words)) + (md5_uint32) 0x6b901122; ++words; (A = (A << 7) | (A >> (32 - 7))); A += B; } while (0);
      do { D += (C ^ (A & (B ^ C))) + (*cwp++ = (*words)) + (md5_uint32) 0xfd987193; ++words; (D = (D << 12) | (D >> (32 - 12))); D += A; } while (0);
      do { C += (B ^ (D & (A ^ B))) + (*cwp++ = (*words)) + (md5_uint32) 0xa679438e; ++words; (C = (C << 17) | (C >> (32 - 17))); C += D; } while (0);
      do { B += (A ^ (C & (D ^ A))) + (*cwp++ = (*words)) + (md5_uint32) 0x49b40821; ++words; (B = (B << 22) | (B >> (32 - 22))); B += C; } while (0);
      do { A += (C ^ (D & (B ^ C))) + correct_words[1] + (md5_uint32) 0xf61e2562; (A = (A << 5) | (A >> (32 - 5))); A += B; } while (0);
      do { D += (B ^ (C & (A ^ B))) + correct_words[6] + (md5_uint32) 0xc040b340; (D = (D << 9) | (D >> (32 - 9))); D += A; } while (0);
      do { C += (A ^ (B & (D ^ A))) + correct_words[11] + (md5_uint32) 0x265e5a51; (C = (C << 14) | (C >> (32 - 14))); C += D; } while (0);
      do { B += (D ^ (A & (C ^ D))) + correct_words[0] + (md5_uint32) 0xe9b6c7aa; (B = (B << 20) | (B >> (32 - 20))); B += C; } while (0);
      do { A += (C ^ (D & (B ^ C))) + correct_words[5] + (md5_uint32) 0xd62f105d; (A = (A << 5) | (A >> (32 - 5))); A += B; } while (0);
      do { D += (B ^ (C & (A ^ B))) + correct_words[10] + (md5_uint32) 0x02441453; (D = (D << 9) | (D >> (32 - 9))); D += A; } while (0);
      do { C += (A ^ (B & (D ^ A))) + correct_words[15] + (md5_uint32) 0xd8a1e681; (C = (C << 14) | (C >> (32 - 14))); C += D; } while (0);
      do { B += (D ^ (A & (C ^ D))) + correct_words[4] + (md5_uint32) 0xe7d3fbc8; (B = (B << 20) | (B >> (32 - 20))); B += C; } while (0);
      do { A += (C ^ (D & (B ^ C))) + correct_words[9] + (md5_uint32) 0x21e1cde6; (A = (A << 5) | (A >> (32 - 5))); A += B; } while (0);
      do { D += (B ^ (C & (A ^ B))) + correct_words[14] + (md5_uint32) 0xc33707d6; (D = (D << 9) | (D >> (32 - 9))); D += A; } while (0);
      do { C += (A ^ (B & (D ^ A))) + correct_words[3] + (md5_uint32) 0xf4d50d87; (C = (C << 14) | (C >> (32 - 14))); C += D; } while (0);
      do { B += (D ^ (A & (C ^ D))) + correct_words[8] + (md5_uint32) 0x455a14ed; (B = (B << 20) | (B >> (32 - 20))); B += C; } while (0);
      do { A += (C ^ (D & (B ^ C))) + correct_words[13] + (md5_uint32) 0xa9e3e905; (A = (A << 5) | (A >> (32 - 5))); A += B; } while (0);
      do { D += (B ^ (C & (A ^ B))) + correct_words[2] + (md5_uint32) 0xfcefa3f8; (D = (D << 9) | (D >> (32 - 9))); D += A; } while (0);
      do { C += (A ^ (B & (D ^ A))) + correct_words[7] + (md5_uint32) 0x676f02d9; (C = (C << 14) | (C >> (32 - 14))); C += D; } while (0);
      do { B += (D ^ (A & (C ^ D))) + correct_words[12] + (md5_uint32) 0x8d2a4c8a; (B = (B << 20) | (B >> (32 - 20))); B += C; } while (0);
      do { A += (B ^ C ^ D) + correct_words[5] + (md5_uint32) 0xfffa3942; (A = (A << 4) | (A >> (32 - 4))); A += B; } while (0);
      do { D += (A ^ B ^ C) + correct_words[8] + (md5_uint32) 0x8771f681; (D = (D << 11) | (D >> (32 - 11))); D += A; } while (0);
      do { C += (D ^ A ^ B) + correct_words[11] + (md5_uint32) 0x6d9d6122; (C = (C << 16) | (C >> (32 - 16))); C += D; } while (0);
      do { B += (C ^ D ^ A) + correct_words[14] + (md5_uint32) 0xfde5380c; (B = (B << 23) | (B >> (32 - 23))); B += C; } while (0);
      do { A += (B ^ C ^ D) + correct_words[1] + (md5_uint32) 0xa4beea44; (A = (A << 4) | (A >> (32 - 4))); A += B; } while (0);
      do { D += (A ^ B ^ C) + correct_words[4] + (md5_uint32) 0x4bdecfa9; (D = (D << 11) | (D >> (32 - 11))); D += A; } while (0);
      do { C += (D ^ A ^ B) + correct_words[7] + (md5_uint32) 0xf6bb4b60; (C = (C << 16) | (C >> (32 - 16))); C += D; } while (0);
      do { B += (C ^ D ^ A) + correct_words[10] + (md5_uint32) 0xbebfbc70; (B = (B << 23) | (B >> (32 - 23))); B += C; } while (0);
      do { A += (B ^ C ^ D) + correct_words[13] + (md5_uint32) 0x289b7ec6; (A = (A << 4) | (A >> (32 - 4))); A += B; } while (0);
      do { D += (A ^ B ^ C) + correct_words[0] + (md5_uint32) 0xeaa127fa; (D = (D << 11) | (D >> (32 - 11))); D += A; } while (0);
      do { C += (D ^ A ^ B) + correct_words[3] + (md5_uint32) 0xd4ef3085; (C = (C << 16) | (C >> (32 - 16))); C += D; } while (0);
      do { B += (C ^ D ^ A) + correct_words[6] + (md5_uint32) 0x04881d05; (B = (B << 23) | (B >> (32 - 23))); B += C; } while (0);
      do { A += (B ^ C ^ D) + correct_words[9] + (md5_uint32) 0xd9d4d039; (A = (A << 4) | (A >> (32 - 4))); A += B; } while (0);
      do { D += (A ^ B ^ C) + correct_words[12] + (md5_uint32) 0xe6db99e5; (D = (D << 11) | (D >> (32 - 11))); D += A; } while (0);
      do { C += (D ^ A ^ B) + correct_words[15] + (md5_uint32) 0x1fa27cf8; (C = (C << 16) | (C >> (32 - 16))); C += D; } while (0);
      do { B += (C ^ D ^ A) + correct_words[2] + (md5_uint32) 0xc4ac5665; (B = (B << 23) | (B >> (32 - 23))); B += C; } while (0);
      do { A += (C ^ (B | ~D)) + correct_words[0] + (md5_uint32) 0xf4292244; (A = (A << 6) | (A >> (32 - 6))); A += B; } while (0);
      do { D += (B ^ (A | ~C)) + correct_words[7] + (md5_uint32) 0x432aff97; (D = (D << 10) | (D >> (32 - 10))); D += A; } while (0);
      do { C += (A ^ (D | ~B)) + correct_words[14] + (md5_uint32) 0xab9423a7; (C = (C << 15) | (C >> (32 - 15))); C += D; } while (0);
      do { B += (D ^ (C | ~A)) + correct_words[5] + (md5_uint32) 0xfc93a039; (B = (B << 21) | (B >> (32 - 21))); B += C; } while (0);
      do { A += (C ^ (B | ~D)) + correct_words[12] + (md5_uint32) 0x655b59c3; (A = (A << 6) | (A >> (32 - 6))); A += B; } while (0);
      do { D += (B ^ (A | ~C)) + correct_words[3] + (md5_uint32) 0x8f0ccc92; (D = (D << 10) | (D >> (32 - 10))); D += A; } while (0);
      do { C += (A ^ (D | ~B)) + correct_words[10] + (md5_uint32) 0xffeff47d; (C = (C << 15) | (C >> (32 - 15))); C += D; } while (0);
      do { B += (D ^ (C | ~A)) + correct_words[1] + (md5_uint32) 0x85845dd1; (B = (B << 21) | (B >> (32 - 21))); B += C; } while (0);
      do { A += (C ^ (B | ~D)) + correct_words[8] + (md5_uint32) 0x6fa87e4f; (A = (A << 6) | (A >> (32 - 6))); A += B; } while (0);
      do { D += (B ^ (A | ~C)) + correct_words[15] + (md5_uint32) 0xfe2ce6e0; (D = (D << 10) | (D >> (32 - 10))); D += A; } while (0);
      do { C += (A ^ (D | ~B)) + correct_words[6] + (md5_uint32) 0xa3014314; (C = (C << 15) | (C >> (32 - 15))); C += D; } while (0);
      do { B += (D ^ (C | ~A)) + correct_words[13] + (md5_uint32) 0x4e0811a1; (B = (B << 21) | (B >> (32 - 21))); B += C; } while (0);
      do { A += (C ^ (B | ~D)) + correct_words[4] + (md5_uint32) 0xf7537e82; (A = (A << 6) | (A >> (32 - 6))); A += B; } while (0);
      do { D += (B ^ (A | ~C)) + correct_words[11] + (md5_uint32) 0xbd3af235; (D = (D << 10) | (D >> (32 - 10))); D += A; } while (0);
      do { C += (A ^ (D | ~B)) + correct_words[2] + (md5_uint32) 0x2ad7d2bb; (C = (C << 15) | (C >> (32 - 15))); C += D; } while (0);
      do { B += (D ^ (C | ~A)) + correct_words[9] + (md5_uint32) 0xeb86d391; (B = (B << 21) | (B >> (32 - 21))); B += C; } while (0);
      A += A_save;
      B += B_save;
      C += C_save;
      D += D_save;
    }
  ctx->A = A;
  ctx->B = B;
  ctx->C = C;
  ctx->D = D;
}
const char *libiberty_optr;
char *libiberty_nptr;
unsigned long libiberty_len;
typedef union hdr
{
  char align[sizeof(double)];
  struct
    {
      union hdr *next;
      char *deep;
    } h;
} header;
static header *last_alloca_header = ((void *)0);
void *
C_alloca (size)
     size_t size;
{
  auto char probe;
  register char *depth = &(probe);
  {
    register header *hp;
    for (hp = last_alloca_header; hp != ((void *)0);)
      if ((-1 > 0 && hp->h.deep > depth)
   || (-1 < 0 && hp->h.deep < depth))
 {
   register header *np = hp->h.next;
   free ((void *) hp);
   hp = np;
 }
      else
 break;
    last_alloca_header = hp;
  }
  if (size == 0)
    return ((void *)0);
  {
    register void * new = xmalloc (sizeof (header) + size);
    if (new == 0)
      abort();
    ((header *) new)->h.next = last_alloca_header;
    ((header *) new)->h.deep = depth;
    last_alloca_header = (header *) new;
    return (void *) ((char *) new + sizeof (header));
  }
}
char *
dupargv (argv)
     char *argv;
{
  int argc;
  char *copy;
  if (argv == ((void *)0))
    return ((void *)0);
  for (argc = 0; argv[argc] != ((void *)0); argc++);
  copy = (char *) malloc ((argc + 1) * sizeof (char *));
  if (copy == ((void *)0))
    return ((void *)0);
  for (argc = 0; argv[argc] != ((void *)0); argc++)
    {
      int len = strlen (argv[argc]);
      copy[argc] = malloc (sizeof (char *) * (len + 1));
      if (copy[argc] == ((void *)0))
 {
   freeargv (copy);
   return ((void *)0);
 }
      strcpy (copy[argc], argv[argc]);
    }
  copy[argc] = ((void *)0);
  return copy;
}
void freeargv (vector)
char *vector;
{
  register char *scan;
  if (vector != ((void *)0))
    {
      for (scan = vector; *scan != ((void *)0); scan++)
 {
   free (*scan);
 }
      free (vector);
    }
}
char *buildargv (input)
     const char *input;
{
  char *arg;
  char *copybuf;
  int squote = 0;
  int dquote = 0;
  int bsquote = 0;
  int argc = 0;
  int maxargc = 0;
  char *argv = ((void *)0);
  char *nargv;
  if (input != ((void *)0))
    {
      copybuf = (char *) C_alloca(strlen (input) + 1);
      do
 {
   while (((*input) == ' ' || (*input) == '\t'))
     {
       input++;
     }
   if ((maxargc == 0) || (argc >= (maxargc - 1)))
     {
       if (argv == ((void *)0))
  {
    maxargc = 8;
    nargv = (char *) malloc (maxargc * sizeof (char *));
  }
       else
  {
    maxargc *= 2;
    nargv = (char *) realloc (argv, maxargc * sizeof (char *));
  }
       if (nargv == ((void *)0))
  {
    if (argv != ((void *)0))
      {
        freeargv (argv);
        argv = ((void *)0);
      }
    break;
  }
       argv = nargv;
       argv[argc] = ((void *)0);
     }
   arg = copybuf;
   while (*input != 0)
     {
       if (((*input) == ' ' || (*input) == '\t') && !squote && !dquote && !bsquote)
  {
    break;
  }
       else
  {
    if (bsquote)
      {
        bsquote = 0;
        *arg++ = *input;
      }
    else if (*input == 0)
      {
        bsquote = 1;
      }
    else if (squote)
      {
        if (*input == 0)
   {
     squote = 0;
   }
        else
   {
     *arg++ = *input;
   }
      }
    else if (dquote)
      {
        if (*input == '"')
   {
     dquote = 0;
   }
        else
   {
     *arg++ = *input;
   }
      }
    else
      {
        if (*input == 0)
   {
     squote = 1;
   }
        else if (*input == '"')
   {
     dquote = 1;
   }
        else
   {
     *arg++ = *input;
   }
      }
    input++;
  }
     }
   *arg = 0;
   argv[argc] = strdup (copybuf);
   if (argv[argc] == ((void *)0))
     {
       freeargv (argv);
       argv = ((void *)0);
       break;
     }
   argc++;
   argv[argc] = ((void *)0);
   while (((*input) == ' ' || (*input) == '\t'))
     {
       input++;
     }
 }
      while (*input != 0);
    }
  return (argv);
}
extern char *choose_tmpdir (void);
char *
choose_temp_base ()
{
  const char *base = choose_tmpdir ();
  char *temp_filename;
  int len;
  len = strlen (base);
  temp_filename = xmalloc (len + (sizeof("ccXXXXXX") - 1) + 1);
  strcpy (temp_filename, base);
  strcpy (temp_filename + len, "ccXXXXXX");
  mktemp (temp_filename);
  if (strlen (temp_filename) == 0)
    abort ();
  return temp_filename;
}
static unsigned long vconcat_length (const char *, va_list);
static unsigned long
vconcat_length (first, args)
     const char *first;
     va_list args;
{
  unsigned long length = 0;
  const char *arg;
  for (arg = first; arg ; arg = __builtin_va_arg(args,c))
    length += strlen (arg);
  return length;
}
static char *vconcat_copy (char *, const char *, va_list);
static char *
vconcat_copy (dst, first, args)
     char *dst;
     const char *first;
     va_list args;
{
  char *end = dst;
  const char *arg;
  for (arg = first; arg ; arg = __builtin_va_arg(args,c))
    {
      unsigned long length = strlen (arg);
      memcpy (end, arg, length);
      end += length;
    }
  *end = 0;
  return dst;
}
unsigned long
concat_length (const char *first, ...)
{
  unsigned long length;
  { va_list args; __builtin_va_start(args,first); { struct Qdmy;
  struct Qdmy;
  length = vconcat_length (first, args);
  } __builtin_va_end(args); };
  return length;
}
char *
concat_copy (char *dst, const char *first, ...)
{
  char *save_dst;
  { va_list args; __builtin_va_start(args,first); { struct Qdmy;
  struct Qdmy;
  struct Qdmy;
  vconcat_copy (dst, first, args);
  save_dst = dst;
  } __builtin_va_end(args); };
  return save_dst;
}
char *libiberty_concat_ptr;
char *
concat_copy2 (const char *first, ...)
{
  { va_list args; __builtin_va_start(args,first); { struct Qdmy;
  struct Qdmy;
  vconcat_copy (libiberty_concat_ptr, first, args);
  } __builtin_va_end(args); };
  return libiberty_concat_ptr;
}
char *
concat (const char *first, ...)
{
  char *newstr;
  { va_list args; __builtin_va_start(args,first); { struct Qdmy;
  struct Qdmy;
  newstr = (char *) xmalloc (vconcat_length (first, args) + 1);
  } __builtin_va_end(args); };
  { va_list args; __builtin_va_start(args,first); { struct Qdmy;
  struct Qdmy;
  vconcat_copy (newstr, first, args);
  } __builtin_va_end(args); };
  return newstr;
}
char *
reconcat (char *optr, const char *first, ...)
{
  char *newstr;
  { va_list args; __builtin_va_start(args,first); { struct Qdmy;
  struct Qdmy;
  struct Qdmy;
  newstr = (char *) xmalloc (vconcat_length (first, args) + 1);
  } __builtin_va_end(args); };
  { va_list args; __builtin_va_start(args,first); { struct Qdmy;
  struct Qdmy;
  struct Qdmy;
  vconcat_copy (newstr, first, args);
  if (optr)
    free (optr);
  } __builtin_va_end(args); };
  return newstr;
}
int
cplus_demangle_fill_component (p, type, left, right)
     struct demangle_component *p;
     enum demangle_component_type type;
     struct demangle_component *left;
     struct demangle_component *right;
{
  if (p == ((void *)0))
    return 0;
  switch (type)
    {
    case DEMANGLE_COMPONENT_QUAL_NAME:
    case DEMANGLE_COMPONENT_LOCAL_NAME:
    case DEMANGLE_COMPONENT_TYPED_NAME:
    case DEMANGLE_COMPONENT_TEMPLATE:
    case DEMANGLE_COMPONENT_CONSTRUCTION_VTABLE:
    case DEMANGLE_COMPONENT_VENDOR_TYPE_QUAL:
    case DEMANGLE_COMPONENT_FUNCTION_TYPE:
    case DEMANGLE_COMPONENT_ARRAY_TYPE:
    case DEMANGLE_COMPONENT_PTRMEM_TYPE:
    case DEMANGLE_COMPONENT_ARGLIST:
    case DEMANGLE_COMPONENT_TEMPLATE_ARGLIST:
    case DEMANGLE_COMPONENT_UNARY:
    case DEMANGLE_COMPONENT_BINARY:
    case DEMANGLE_COMPONENT_BINARY_ARGS:
    case DEMANGLE_COMPONENT_TRINARY:
    case DEMANGLE_COMPONENT_TRINARY_ARG1:
    case DEMANGLE_COMPONENT_TRINARY_ARG2:
    case DEMANGLE_COMPONENT_LITERAL:
    case DEMANGLE_COMPONENT_LITERAL_NEG:
      break;
    case DEMANGLE_COMPONENT_VTABLE:
    case DEMANGLE_COMPONENT_VTT:
    case DEMANGLE_COMPONENT_TYPEINFO:
    case DEMANGLE_COMPONENT_TYPEINFO_NAME:
    case DEMANGLE_COMPONENT_TYPEINFO_FN:
    case DEMANGLE_COMPONENT_THUNK:
    case DEMANGLE_COMPONENT_VIRTUAL_THUNK:
    case DEMANGLE_COMPONENT_COVARIANT_THUNK:
    case DEMANGLE_COMPONENT_JAVA_CLASS:
    case DEMANGLE_COMPONENT_GUARD:
    case DEMANGLE_COMPONENT_REFTEMP:
    case DEMANGLE_COMPONENT_RESTRICT:
    case DEMANGLE_COMPONENT_VOLATILE:
    case DEMANGLE_COMPONENT_CONST:
    case DEMANGLE_COMPONENT_RESTRICT_THIS:
    case DEMANGLE_COMPONENT_VOLATILE_THIS:
    case DEMANGLE_COMPONENT_CONST_THIS:
    case DEMANGLE_COMPONENT_POINTER:
    case DEMANGLE_COMPONENT_REFERENCE:
    case DEMANGLE_COMPONENT_COMPLEX:
    case DEMANGLE_COMPONENT_IMAGINARY:
    case DEMANGLE_COMPONENT_VENDOR_TYPE:
    case DEMANGLE_COMPONENT_CAST:
      if (right != ((void *)0))
 return 0;
      break;
    default:
      return 0;
    }
  p->type = type;
  p->u.s_binary.left = left;
  p->u.s_binary.right = right;
  return 1;
}
int
cplus_demangle_fill_builtin_type (p, typename)
     struct demangle_component *p;
     const char *typename;
{
  int len;
  unsigned int i;
  if (p == ((void *)0) || typename == ((void *)0))
    return 0;
  len = strlen (typename);
  for (i = 0; i < (26); ++i)
    {
      if (len == cplus_demangle_builtin_types[i].len
   && strcmp (typename, cplus_demangle_builtin_types[i].name) == 0)
 {
   p->type = DEMANGLE_COMPONENT_BUILTIN_TYPE;
   p->u.s_builtin.type = &cplus_demangle_builtin_types[i];
   return 1;
 }
    }
  return 0;
}
int
cplus_demangle_fill_operator (p, opname, args)
     struct demangle_component *p;
     const char *opname;
     int args;
{
  int len;
  unsigned int i;
  if (p == ((void *)0) || opname == ((void *)0))
    return 0;
  len = strlen (opname);
  for (i = 0; cplus_demangle_operators[i].name != ((void *)0); ++i)
    {
      if (len == cplus_demangle_operators[i].len
   && args == cplus_demangle_operators[i].args
   && strcmp (opname, cplus_demangle_operators[i].name) == 0)
 {
   p->type = DEMANGLE_COMPONENT_OPERATOR;
   p->u.s_operator.op = &cplus_demangle_operators[i];
   return 1;
 }
    }
  return 0;
}
struct demangle_component *
cplus_demangle_v3_components (mangled, options, mem)
     const char *mangled;
     int options;
     void *mem;
{
  size_t len;
  int type;
  struct d_info di;
  struct demangle_component *dc;
  len = strlen (mangled);
  if (mangled[0] == '_' && mangled[1] == 'Z')
    type = 0;
  else
    {
      if ((options & (1 << 4)) == 0)
 return ((void *)0);
      type = 1;
    }
  cplus_demangle_init_info (mangled, options, len, &di);
  di.comps = ((struct demangle_component *)
       malloc (di.num_comps * sizeof (struct demangle_component)));
  di.subs = ((struct demangle_component *)
      malloc (di.num_subs * sizeof (struct demangle_component *)));
  if (di.comps == ((void *)0) || di.subs == ((void *)0))
    {
      if (di.comps != ((void *)0))
 free (di.comps);
      if (di.subs != ((void *)0))
 free (di.subs);
      return ((void *)0);
    }
  if (! type)
    dc = cplus_demangle_mangled_name (&di, 1);
  else
    dc = cplus_demangle_type (&di);
  if ((options & (1 << 0)) != 0 && (*((&di)->n)) != 0)
    dc = ((void *)0);
  free (di.subs);
  if (dc != ((void *)0))
    *mem = di.comps;
  else
    free (di.comps);
  return dc;
}
int
dyn_string_init (ds_struct_ptr, space)
     struct dyn_string *ds_struct_ptr;
     int space;
{
  if (space == 0)
    space = 1;
  ds_struct_ptr->s = (char *) xmalloc (space);
  ds_struct_ptr->allocated = space;
  ds_struct_ptr->length = 0;
  ds_struct_ptr->s[0] = 0;
  return 1;
}
dyn_string_t
dyn_string_new (space)
     int space;
{
  dyn_string_t result;
  result = (dyn_string_t) xmalloc (sizeof (struct dyn_string));
  dyn_string_init (result, space);
  return result;
}
void
dyn_string_delete (ds)
     dyn_string_t ds;
{
  free (ds->s);
  free (ds);
}
char*
dyn_string_release (ds)
     dyn_string_t ds;
{
  char* result = ds->s;
  ds->s = ((void *)0);
  free (ds);
  return result;
}
dyn_string_t
dyn_string_resize (ds, space)
     dyn_string_t ds;
     int space;
{
  int new_allocated = ds->allocated;
  ++space;
  while (space > new_allocated)
    new_allocated *= 2;
  if (new_allocated != ds->allocated)
    {
      ds->allocated = new_allocated;
      ds->s = (char *) xrealloc (ds->s, ds->allocated);
    }
  return ds;
}
void
dyn_string_clear (ds)
     dyn_string_t ds;
{
  ds->s[0] = 0;
  ds->length = 0;
}
int
dyn_string_copy (dest, src)
     dyn_string_t dest;
     dyn_string_t src;
{
  if (dest == src)
    abort ();
  if (dyn_string_resize (dest, src->length) == ((void *)0))
    return 0;
  strcpy (dest->s, src->s);
  dest->length = src->length;
  return 1;
}
int
dyn_string_copy_cstr (dest, src)
     dyn_string_t dest;
     const char *src;
{
  int length = strlen (src);
  if (dyn_string_resize (dest, length) == ((void *)0))
    return 0;
  strcpy (dest->s, src);
  dest->length = length;
  return 1;
}
int
dyn_string_prepend (dest, src)
     dyn_string_t dest;
     dyn_string_t src;
{
  return dyn_string_insert (dest, 0, src);
}
int
dyn_string_prepend_cstr (dest, src)
     dyn_string_t dest;
     const char *src;
{
  return dyn_string_insert_cstr (dest, 0, src);
}
int
dyn_string_insert (dest, pos, src)
     dyn_string_t dest;
     int pos;
     dyn_string_t src;
{
  int i;
  if (src == dest)
    abort ();
  if (dyn_string_resize (dest, dest->length + src->length) == ((void *)0))
    return 0;
  for (i = dest->length; i >= pos; --i)
    dest->s[i + src->length] = dest->s[i];
  strncpy (dest->s + pos, src->s, src->length);
  dest->length += src->length;
  return 1;
}
int
dyn_string_insert_cstr (dest, pos, src)
     dyn_string_t dest;
     int pos;
     const char *src;
{
  int i;
  int length = strlen (src);
  if (dyn_string_resize (dest, dest->length + length) == ((void *)0))
    return 0;
  for (i = dest->length; i >= pos; --i)
    dest->s[i + length] = dest->s[i];
  strncpy (dest->s + pos, src, length);
  dest->length += length;
  return 1;
}
int
dyn_string_insert_char (dest, pos, c)
     dyn_string_t dest;
     int pos;
     int c;
{
  int i;
  if (dyn_string_resize (dest, dest->length + 1) == ((void *)0))
    return 0;
  for (i = dest->length; i >= pos; --i)
    dest->s[i + 1] = dest->s[i];
  dest->s[pos] = c;
  ++dest->length;
  return 1;
}
int
dyn_string_append (dest, s)
     dyn_string_t dest;
     dyn_string_t s;
{
  if (dyn_string_resize (dest, dest->length + s->length) == 0)
    return 0;
  strcpy (dest->s + dest->length, s->s);
  dest->length += s->length;
  return 1;
}
int
dyn_string_append_cstr (dest, s)
     dyn_string_t dest;
     const char *s;
{
  int len = strlen (s);
  if (dyn_string_resize (dest, dest->length + len) == ((void *)0))
    return 0;
  strcpy (dest->s + dest->length, s);
  dest->length += len;
  return 1;
}
int
dyn_string_append_char (dest, c)
     dyn_string_t dest;
     int c;
{
  if (dyn_string_resize (dest, dest->length + 1) == ((void *)0))
    return 0;
  dest->s[dest->length] = c;
  dest->s[dest->length + 1] = 0;
  ++(dest->length);
  return 1;
}
int
dyn_string_substring (dest, src, start, end)
     dyn_string_t dest;
     dyn_string_t src;
     int start;
     int end;
{
  int i;
  int length = end - start;
  if (start > end || start > src->length || end > src->length)
    abort ();
  if (dyn_string_resize (dest, length) == ((void *)0))
    return 0;
  for (i = length; --i >= 0; )
    dest->s[i] = src->s[start + i];
  dest->s[length] = 0;
  dest->length = length;
  return 1;
}
int
dyn_string_eq (ds1, ds2)
     dyn_string_t ds1;
     dyn_string_t ds2;
{
  if (ds1->length != ds2->length)
    return 0;
  else
    return !strcmp (ds1->s, ds2->s);
}
struct stat
  {
    __dev_t st_dev;
    __ino_t st_ino;
    __nlink_t st_nlink;
    __mode_t st_mode;
    __uid_t st_uid;
    __gid_t st_gid;
    int __pad0;
    __dev_t st_rdev;
    __off_t st_size;
    __blksize_t st_blksize;
    __blkcnt_t st_blocks;
    struct timespec st_atim;
    struct timespec st_mtim;
    struct timespec st_ctim;
    __syscall_slong_t __glibc_reserved[3];
  };
struct stat64
  {
    __dev_t st_dev;
    __ino64_t st_ino;
    __nlink_t st_nlink;
    __mode_t st_mode;
    __uid_t st_uid;
    __gid_t st_gid;
    int __pad0;
    __dev_t st_rdev;
    __off_t st_size;
    __blksize_t st_blksize;
    __blkcnt64_t st_blocks;
    struct timespec st_atim;
    struct timespec st_mtim;
    struct timespec st_ctim;
    __syscall_slong_t __glibc_reserved[3];
  };
extern int stat (const char * __file,
   struct stat * __buf) ;
extern int fstat (int __fd, struct stat *__buf) ;
extern int stat64 (const char * __file,
     struct stat64 * __buf) ;
extern int fstat64 (int __fd, struct stat64 *__buf) ;
extern int fstatat (int __fd, const char * __file,
      struct stat * __buf, int __flag)
     ;
extern int fstatat64 (int __fd, const char * __file,
        struct stat64 * __buf, int __flag)
     ;
extern int lstat (const char * __file,
    struct stat * __buf) ;
extern int lstat64 (const char * __file,
      struct stat64 * __buf)
     ;
