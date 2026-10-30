## $Id$

include $(top_srcdir)/Makefile.am.global

SUBDIRS = html man

# API documentation via Doxygen, generated on demand only (not part of all/check).
DOT = @DOT@

Doxyfile: $(srcdir)/Doxyfile.in Makefile
	if test -n "$(DOT)"; then have_dot=YES; dot_path=`dirname "$(DOT)"`; \
	else have_dot=NO; dot_path=; fi; \
	strip_path=`dirname "$(top_srcdir)"`; \
	sed -e 's|@top_srcdir@|$(top_srcdir)|g' \
	    -e "s|@HAVE_DOT@|$$have_dot|g" \
	    -e "s|@DOT_PATH@|$$dot_path|g" \
	    -e "s|@STRIP_FROM_PATH@|$$strip_path|g" \
	    -e 's|@PROJECT_NUMBER@|$(PACKAGE_VERSION)|g' \
	    $(srcdir)/Doxyfile.in > $@

doxygen: Doxyfile
	doxygen Doxyfile

.PHONY: doxygen
