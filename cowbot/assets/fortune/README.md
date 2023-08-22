The datfiles copied here are from [shlomif/fortune-mod](https://github.com/shlomif/fortune-mod).

If you would like to add a fortune please check with upstream.

---

# LICENSES

/*-
 * Copyright (c) 1986, 1993
 *	The Regents of the University of California.  All rights reserved.
 *
 * This code is derived from software contributed to Berkeley by
 * Ken Arnold.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. All advertising materials mentioning features or use of this software
 *    must display the following acknowledgement:
 *	This product includes software developed by the University of
 *	California, Berkeley and its contributors.
 * 4. Neither the name of the University nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */

/* Modified September, 1995, Amy A. Lewis
 * 1: removed all file-locking dreck.  Unnecessary
 * 2: Fixed bug that made fortune -f report a different list than
 *    fortune with any other parameters, or none, and which forced
 *    the program to read only one file (named 'fortunes')
 * 3: removed the unnecessary print_file_list()
 * 4: Added "OFFDIR" to pathnames.h as the directory in which offensive
 *    fortunes are kept.  This considerably simplifies our life by
 *    permitting us to dispense with a lot of silly tests for the string
 *    "-o" at the end of a filename.
 * 5: I think the problems with trying to find filenames were fixed by
 *    the change in the way that offensive files are defined.  Two birds,
 *    one stone!
 * 6: Calculated probabilities for all files, so that -f will print them.
 */

/* Changes Copyright (c) 1997 Dennis L. Clark.  All rights reserved.
 *
 *    The changes in this file may be freely redistributed, modified or
 *    included in other software, as long as both the above copyright
 *    notice and these conditions appear intact.
 */

/* Modified May 1997, Dennis L. Clark (dbugger@progsoc.uts.edu.au)
 *  + Various portability fixes
 *  + Percent selection of files with -a now works on datafiles which
 *    appear in both unoffensive and offensive directories (see man page
 *    for details)
 *  + The -s and -l options are now more consistent in their
 *    interpretation of fortune length
 *  + The -s and -l options can now be combined wit the -m option
 */

/* Modified Jul 1999, Pablo Saratxaga <srtxg@chanae.alphanet.ch>
 * - added use of the LANG variables; now if called without argument
 * it will choose (if they exist) fortunes in the users' language.
 * (that is, under a directory $LANG/ under the main fortunes directory
 *
 * Added to debian by Alastair McKinstry, <mckinstry@computer.org>, 2002-07-31
 */

